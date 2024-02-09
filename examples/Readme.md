# Examples

Here are some curl examples for using terakube:

``
curl --request POST \
--url http://localhost:3000/api/templates \
--header 'Content-Type: application/json' \
--header 'User-Agent: insomnia/2023.5.8' \
--data '{
"content": "{\"apiVersion\":\"batch/v1\",\"kind\":\"Job\",\"metadata\":{\"generateName\":\"loco-generator\",\"labels\":{\"app\":\"terakube\"}},\"spec\":{\"template\":{\"metadata\":{\"name\":\"jq-postgres-job\"},\"spec\":{\"initContainers\":[{\"name\":\"jq-init\",\"image\":\"efrecon/jq:1.7.1\",\"command\":[\"/bin/sh\",\"-c\",\"echo '\''def map_custom_type:\\n  if .type == \\\"string\\\" then\\n    if .format == \\\"uuid\\\" then .type = \\\"uuid\\\"\\n    else\\n      .type = \\\"string\\\"\\n    end\\n  elif .type == \\\"integer\\\" then\\n    if .length <= 255 then .type = \\\"tiny_integer\\\"\\n    elif .length <= 65535 then .type = \\\"small_integer\\\"\\n    elif .length <= 4294967295 then .type = \\\"int\\\"\\n    else .type = \\\"big_integer\\\"\\n    end\\n  elif .type == \\\"number\\\" then\\n    .type = \\\"float\\\"\\n  else\\n    .\\n  end;\\n\\ndef handle_nullable:\\n  if .type | type == \\\"array\\\" and index(\\\"null\\\") then\\n    .type = (.type - [\\\"null\\\"] | first) |\\n    .nullable = true\\n  else\\n    .\\n  end;\\n\\ndef generate_loco_command:\\n  \\\"cargo loco generate model \\\" + .title + \\\" \\\" + (.properties | to_entries | map({name: .key} + (.value | handle_nullable | map_custom_type)) | map(.name + \\\":\\\" + .type + (if .nullable then \\\"!\\\" else \\\"\\\" end)) | join(\\\" \\\"));\\n\\ngenerate_loco_command'\'' > /data/jq-functions.jq && cat /data/jq-functions.jq\"],\"volumeMounts\":[{\"name\":\"shared-data\",\"mountPath\":\"/data\"}]},{\"name\":\"generate-script\",\"image\":\"efrecon/jq:1.7.1\",\"command\":[\"/bin/sh\",\"-c\",\"curl -s '\''https://raw.githubusercontent.com/dinosath/oasgen-k8s/main/article.json'\'' | jq -f /data/jq-functions.jq  > /data/loco.sh && chmod +x /data/loco.sh && cat /data/loco.sh\"],\"volumeMounts\":[{\"name\":\"shared-data\",\"mountPath\":\"/data\"}]},{\"name\":\"generate-script\",\"image\":\"efrecon/jq:1.7.1\",\"command\":[\"/bin/sh\",\"-c\",\"for url in $(jq -r '\''.\\\"json-schemas\\\"[]'\'' /data/schemas.json); do if [ -n \\\"$url\\\" ]; then curl -s \\\"$url\\\" | jq -f /data/jq-functions.jq > \\\"/data/scripts/$(basename $url).sh\\\" && chmod +x \\\"/data/scripts/$(basename $url).sh\\\"; fi; done && echo '\''LOCO_APP_NAME={{application_name}} LOCO_TEMPLATE=saas loco new'\'' > /data/scripts/loco.sh && chmod +x /data/scripts/loco.sh\"],\"volumeMounts\":[{\"name\":\"shared-data\",\"mountPath\":\"/data\"}]}],\"containers\":[{\"name\":\"postgres\",\"image\":\"postgres:latest\",\"env\":[{\"name\":\"POSTGRES_DB\",\"value\":\"your_db\"},{\"name\":\"POSTGRES_USER\",\"value\":\"your_user\"},{\"name\":\"POSTGRES_PASSWORD\",\"value\":\"your_password\"}],\"ports\":[{\"containerPort\":5432}]},{\"name\":\"loco\",\"image\":\"ghcr.io/dinosath/loco-cli\",\"command\":[\"/bin/bash\",\"/scripts/generated-script.sh\"],\"volumeMounts\":[{\"name\":\"shared-data\",\"mountPath\":\"/data\"}]}],\"volumes\":[{\"name\":\"shared-data\",\"emptyDir\":{}}],\"restartPolicy\":\"Never\"}},\"backoffLimit\":4}}"
}'
``
