#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::_entities::templates::{ActiveModel, Entity, Model};
use tera::{Context, Tera};
use k8s_openapi::api::batch::v1::Job;
use k8s_openapi::api::core::v1::{PersistentVolume, Pod};

use kube::{
    api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams, ResourceExt},
    runtime::wait::{await_condition, conditions, conditions::is_pod_running},
    Client,
};

use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post, delete},
    Router,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub content: String,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.content = Set(self.content.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<Model>>> {
    format::json(Entity::find().all(&ctx.db).await?)
}

pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Json<Model>> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Json<Model>> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

pub async fn renderById(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(body): Json<Value>,
) -> Result<String> {
    let template = load_item(&ctx, id).await?;
    render_template_with_context(&*template.content, body)
}

#[derive(Serialize, Deserialize, Debug)]
struct RenderTemplateRequest {
    data: Data,
    template: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    application_name: String,
    jsonSchemas: Vec<String>,
}
pub async fn render(
    State(ctx): State<AppContext>,
    Json(body): Json<RenderTemplateRequest>,
) -> Result<String> {
    let data = serde_json::to_value(&body.data).unwrap();
    render_template_with_context(&*body.template, data)
}

pub async fn create_job(
    State(ctx): State<AppContext>,
    Json(body): Json<RenderTemplateRequest>,
) -> Result<String> {
    let data = serde_json::to_value(&body.data).unwrap();
    let job_str = render_template_with_context(&*body.template, data)?;

    let client = Client::try_default().await.map_err(|e| {
        let msg = format!("Failed to create Kubernetes client: {}", e);
        Error::Message(msg)
    })?;

    let jobs: Api<Job> = Api::default_namespaced(client);
    // Create the job using the Kubernetes API
    let post_params = PostParams::default();

    // Deserialize the job string into a Job object
    let job: Job = match serde_json::from_str(&job_str) {
        Ok(job) => job,
        Err(e) => {
            let msg = format!("Failed to deserialize job: {}", e);
            return Err(Error::Message(msg));
        }
    };

    match jobs.create(&post_params, &job).await {
        Ok(_) => {
            Ok("Job created successfully".to_string())
        }
        Err(err) => {
            let msg = format!("Failed to create job: {}", err);
            Err(Error::Message(msg))
        }
    }
}
pub async fn create_job_by_id(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(body): Json<Value>,
) -> Result<String> {
    let template = load_item(&ctx, id).await?;
    let job_str = render_template_with_context(&*template.content, body)?;

    let client = Client::try_default().await.map_err(|e| {
        let msg = format!("Failed to create Kubernetes client: {}", e);
        Error::Message(msg)
    })?;

    let jobs: Api<Job> = Api::default_namespaced(client);
    // Create the job using the Kubernetes API
    let post_params = PostParams::default();

    // Deserialize the job string into a Job object
    let job: Job = match serde_json::from_str(&job_str) {
        Ok(job) => job,
        Err(e) => {
            let msg = format!("Failed to deserialize job: {}", e);
            return Err(Error::Message(msg));
        }
    };

    match jobs.create(&post_params, &job).await {
        Ok(_) => {
            Ok("Job created successfully".to_string())
        }
        Err(err) => {
            let msg = format!("Failed to create job: {}", err);
            Err(Error::Message(msg))
        }
    }
}

fn render_template_with_context(template_str: &str, context_json: Value) -> Result<String> {
    let tera = match Tera::one_off(template_str, &Context::from_value(context_json)?, false) {
        Ok(rendered) => rendered,
        Err(err) => return Err(Error::from(err)),
    };
    Ok(tera)
}


pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<()> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Json<Model>> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("templates")
        .add("/render", post(render))
        .add("/createjob", post(create_job))
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
        .add("/:id/render", post(renderById))
        .add("/:id/createjob", post(create_job_by_id))
}