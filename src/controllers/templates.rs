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
    routing::{get, post},
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

pub async fn render(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(body): Json<Value>,
) -> Result<String> {
    let template = load_item(&ctx, id).await?;
    render_template_with_context(&*template.content, body)
}

pub async fn create_job(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(body): Json<Value>,
) -> Result<String> {
    let template = load_item(&ctx, id).await?;
    // let user = match res {
    //     Ok(user) => user,
    //     Err(err) => {
    //         tracing::info!(
    //             message = err.to_string(),
    //             user_email = &params.email,
    //             "could not register user",
    //         );
    //         return format::json(());
    //     }
    // };
    let job = render_template_with_context(&*template.content, body);

    let client = Client::try_default().await?;
    let jobs: Api<Job> = Api::default_namespaced(client);
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
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
        .add("/:id/render", post(render))
        .add("/:id/createjob", post(create_job))
}


struct KubeError(kube::Error);


impl IntoResponse for KubeError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// impl<E> From<E> for KubeError
//     where
//         E: Into<loco_rs::Error>
// {
//     fn from(err: E) -> Self {
//         loco_rs::Error::string(format!("Kubernetes error: {}", err.into()).clone().as_str())
//
//     }
// }
//
