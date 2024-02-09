def map_custom_type:
  if .type == "string" then
    if .format == "uuid" then .type = "uuid"
    else
      .type = "string"
    end
  elif .type == "integer" then
    if .length <= 255 then .type = "tiny_integer"
    elif .length <= 65535 then .type = "small_integer"
    elif .length <= 4294967295 then .type = "int"
    else .type = "big_integer"
    end
  elif .type == "number" then
    .type = "float"
  else
    .
  end;

def handle_nullable:
  if .type | type == "array" and index("null") then
    .type = (.type - ["null"] | first) |
    .nullable = true
  else
    .
  end;

def generate_loco_command:
  "cargo loco generate model " + .title + " " + (.properties | to_entries | map({name: .key} + (.value | handle_nullable | map_custom_type)) | map(.name + ":" + .type + (if .nullable then "!" else "" end)) | join(" "));

generate_loco_command