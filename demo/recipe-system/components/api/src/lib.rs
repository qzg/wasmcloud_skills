wit_bindgen::generate!({
    world: "recipe-api",
    path: "../wit",
    exports: {
        "wasi:http/incoming-handler": Component,
    },
});

use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;
use wasi::keyvalue::store::*;
use wasi::logging::logging::*;

use serde::{Deserialize, Serialize};

struct Component;

// Helper struct for JSON serialization
#[derive(Serialize, Deserialize)]
struct RecipeJson {
    id: String,
    name: String,
    description: Option<String>,
    ingredients: Vec<IngredientJson>,
    instructions: Vec<StepJson>,
    servings: u8,
    prep_time_mins: u32,
    cook_time_mins: u32,
    difficulty: String,
    tags: Vec<String>,
    dietary_info: Vec<String>,
    created_at: u64,
    updated_at: u64,
}

#[derive(Serialize, Deserialize)]
struct IngredientJson {
    name: String,
    amount: f32,
    unit: String,
    optional: bool,
    notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct StepJson {
    order: u8,
    instruction: String,
    duration_mins: Option<u32>,
}

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let path_with_query = request.path_with_query().unwrap_or("/".to_string());

        log(Level::Info, "recipe-api", &format!("Request: {}", path_with_query));

        // Parse path
        let parts: Vec<&str> = path_with_query.split('?').collect();
        let path = parts[0];
        let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        // Route request
        match request.method() {
            Method::Get => handle_get(&path_segments, response_out),
            Method::Post => handle_post(&path_segments, request, response_out),
            Method::Put => handle_put(&path_segments, request, response_out),
            Method::Delete => handle_delete(&path_segments, response_out),
            _ => send_response(405, b"Method Not Allowed", response_out),
        }
    }
}

fn handle_get(path: &[&str], response_out: ResponseOutparam) {
    match path {
        ["api", "recipes"] => {
            // List all recipes
            match list_recipes() {
                Ok(recipes) => {
                    let json = serde_json::to_string(&recipes).unwrap();
                    send_json_response(200, json.as_bytes(), response_out);
                }
                Err(e) => {
                    log(Level::Error, "recipe-api", &format!("Error listing recipes: {:?}", e));
                    send_response(500, b"Internal Server Error", response_out);
                }
            }
        }
        ["api", "recipes", id] => {
            // Get specific recipe
            match get_recipe(id) {
                Ok(Some(recipe)) => {
                    let json = serde_json::to_string(&recipe).unwrap();
                    send_json_response(200, json.as_bytes(), response_out);
                }
                Ok(None) => {
                    send_response(404, b"Recipe not found", response_out);
                }
                Err(e) => {
                    log(Level::Error, "recipe-api", &format!("Error getting recipe: {:?}", e));
                    send_response(500, b"Internal Server Error", response_out);
                }
            }
        }
        ["health"] => {
            send_json_response(200, b"{\"status\":\"healthy\"}", response_out);
        }
        _ => {
            send_response(404, b"Not Found", response_out);
        }
    }
}

fn handle_post(path: &[&str], request: IncomingRequest, response_out: ResponseOutparam) {
    match path {
        ["api", "recipes"] => {
            // Create new recipe
            match read_request_body(request) {
                Ok(body) => {
                    match serde_json::from_slice::<RecipeJson>(&body) {
                        Ok(recipe_json) => {
                            match create_recipe(recipe_json) {
                                Ok(id) => {
                                    let response = format!("{{\"id\":\"{}\"}}", id);
                                    send_json_response(201, response.as_bytes(), response_out);
                                }
                                Err(e) => {
                                    log(Level::Error, "recipe-api", &format!("Error creating recipe: {:?}", e));
                                    send_response(500, b"Internal Server Error", response_out);
                                }
                            }
                        }
                        Err(e) => {
                            log(Level::Error, "recipe-api", &format!("Invalid JSON: {:?}", e));
                            send_response(400, b"Invalid JSON", response_out);
                        }
                    }
                }
                Err(_) => {
                    send_response(400, b"Failed to read body", response_out);
                }
            }
        }
        _ => {
            send_response(404, b"Not Found", response_out);
        }
    }
}

fn handle_put(path: &[&str], request: IncomingRequest, response_out: ResponseOutparam) {
    match path {
        ["api", "recipes", id] => {
            // Update recipe
            match read_request_body(request) {
                Ok(body) => {
                    match serde_json::from_slice::<RecipeJson>(&body) {
                        Ok(mut recipe_json) => {
                            recipe_json.id = id.to_string();
                            match update_recipe(id, recipe_json) {
                                Ok(_) => {
                                    send_json_response(200, b"{\"status\":\"updated\"}", response_out);
                                }
                                Err(e) => {
                                    log(Level::Error, "recipe-api", &format!("Error updating recipe: {:?}", e));
                                    send_response(500, b"Internal Server Error", response_out);
                                }
                            }
                        }
                        Err(e) => {
                            log(Level::Error, "recipe-api", &format!("Invalid JSON: {:?}", e));
                            send_response(400, b"Invalid JSON", response_out);
                        }
                    }
                }
                Err(_) => {
                    send_response(400, b"Failed to read body", response_out);
                }
            }
        }
        _ => {
            send_response(404, b"Not Found", response_out);
        }
    }
}

fn handle_delete(path: &[&str], response_out: ResponseOutparam) {
    match path {
        ["api", "recipes", id] => {
            match delete_recipe(id) {
                Ok(_) => {
                    send_json_response(200, b"{\"status\":\"deleted\"}", response_out);
                }
                Err(e) => {
                    log(Level::Error, "recipe-api", &format!("Error deleting recipe: {:?}", e));
                    send_response(500, b"Internal Server Error", response_out);
                }
            }
        }
        _ => {
            send_response(404, b"Not Found", response_out);
        }
    }
}

fn list_recipes() -> Result<Vec<RecipeJson>, String> {
    let bucket = open("recipes").map_err(|e| format!("Failed to open bucket: {:?}", e))?;

    // Get list of recipe IDs
    let ids_bytes = bucket.get("_recipe_ids").map_err(|e| format!("Failed to get IDs: {:?}", e))?;

    let mut recipes = Vec::new();

    if let Some(data) = ids_bytes {
        let ids_str = String::from_utf8(data).map_err(|e| format!("Invalid UTF-8: {:?}", e))?;
        let ids: Vec<String> = serde_json::from_str(&ids_str).unwrap_or_default();

        for id in ids {
            if let Ok(Some(recipe)) = get_recipe(&id) {
                recipes.push(recipe);
            }
        }
    }

    Ok(recipes)
}

fn get_recipe(id: &str) -> Result<Option<RecipeJson>, String> {
    let bucket = open("recipes").map_err(|e| format!("Failed to open bucket: {:?}", e))?;

    let key = format!("recipe:{}", id);
    let data = bucket.get(&key).map_err(|e| format!("Failed to get recipe: {:?}", e))?;

    match data {
        Some(bytes) => {
            let recipe = serde_json::from_slice(&bytes)
                .map_err(|e| format!("Failed to deserialize: {:?}", e))?;
            Ok(Some(recipe))
        }
        None => Ok(None),
    }
}

fn create_recipe(mut recipe: RecipeJson) -> Result<String, String> {
    let bucket = open("recipes").map_err(|e| format!("Failed to open bucket: {:?}", e))?;

    // Generate ID if not provided
    if recipe.id.is_empty() {
        recipe.id = format!("recipe_{}", current_timestamp());
    }

    // Set timestamps
    let now = current_timestamp();
    recipe.created_at = now;
    recipe.updated_at = now;

    // Store recipe
    let key = format!("recipe:{}", recipe.id);
    let data = serde_json::to_vec(&recipe).map_err(|e| format!("Failed to serialize: {:?}", e))?;
    bucket.set(&key, &data).map_err(|e| format!("Failed to store recipe: {:?}", e))?;

    // Update recipe IDs list
    add_recipe_id(&bucket, &recipe.id)?;

    Ok(recipe.id.clone())
}

fn update_recipe(id: &str, recipe: RecipeJson) -> Result<(), String> {
    let bucket = open("recipes").map_err(|e| format!("Failed to open bucket: {:?}", e))?;

    let key = format!("recipe:{}", id);
    let data = serde_json::to_vec(&recipe).map_err(|e| format!("Failed to serialize: {:?}", e))?;
    bucket.set(&key, &data).map_err(|e| format!("Failed to update recipe: {:?}", e))?;

    Ok(())
}

fn delete_recipe(id: &str) -> Result<(), String> {
    let bucket = open("recipes").map_err(|e| format!("Failed to open bucket: {:?}", e))?;

    let key = format!("recipe:{}", id);
    bucket.delete(&key).map_err(|e| format!("Failed to delete recipe: {:?}", e))?;

    // Remove from recipe IDs list
    remove_recipe_id(&bucket, id)?;

    Ok(())
}

fn add_recipe_id(bucket: &Bucket, id: &str) -> Result<(), String> {
    let ids_bytes = bucket.get("_recipe_ids").map_err(|e| format!("Failed to get IDs: {:?}", e))?;

    let mut ids: Vec<String> = if let Some(data) = ids_bytes {
        let ids_str = String::from_utf8(data).map_err(|e| format!("Invalid UTF-8: {:?}", e))?;
        serde_json::from_str(&ids_str).unwrap_or_default()
    } else {
        Vec::new()
    };

    if !ids.contains(&id.to_string()) {
        ids.push(id.to_string());
        let ids_json = serde_json::to_vec(&ids).map_err(|e| format!("Failed to serialize IDs: {:?}", e))?;
        bucket.set("_recipe_ids", &ids_json).map_err(|e| format!("Failed to store IDs: {:?}", e))?;
    }

    Ok(())
}

fn remove_recipe_id(bucket: &Bucket, id: &str) -> Result<(), String> {
    let ids_bytes = bucket.get("_recipe_ids").map_err(|e| format!("Failed to get IDs: {:?}", e))?;

    if let Some(data) = ids_bytes {
        let ids_str = String::from_utf8(data).map_err(|e| format!("Invalid UTF-8: {:?}", e))?;
        let mut ids: Vec<String> = serde_json::from_str(&ids_str).unwrap_or_default();

        ids.retain(|i| i != id);

        let ids_json = serde_json::to_vec(&ids).map_err(|e| format!("Failed to serialize IDs: {:?}", e))?;
        bucket.set("_recipe_ids", &ids_json).map_err(|e| format!("Failed to store IDs: {:?}", e))?;
    }

    Ok(())
}

fn read_request_body(request: IncomingRequest) -> Result<Vec<u8>, String> {
    let body = request.consume().map_err(|_| "Failed to consume body")?;
    let stream = body.stream().map_err(|_| "Failed to get stream")?;

    let mut result = Vec::new();
    loop {
        match stream.blocking_read(4096) {
            Ok(chunk) => {
                if chunk.is_empty() {
                    break;
                }
                result.extend_from_slice(&chunk);
            }
            Err(_) => break,
        }
    }

    Ok(result)
}

fn send_response(status: u16, body: &[u8], response_out: ResponseOutparam) {
    let response = OutgoingResponse::new(Fields::new());
    response.set_status_code(status).unwrap();

    let response_body = response.body().unwrap();
    {
        let stream = response_body.write().unwrap();
        stream.blocking_write_and_flush(body).unwrap();
    }

    OutgoingBody::finish(response_body, None).unwrap();
    ResponseOutparam::set(response_out, Ok(response));
}

fn send_json_response(status: u16, body: &[u8], response_out: ResponseOutparam) {
    let headers = Fields::new();
    headers.set(&"content-type".to_string(), &[b"application/json".to_vec()]).unwrap();

    let response = OutgoingResponse::new(headers);
    response.set_status_code(status).unwrap();

    let response_body = response.body().unwrap();
    {
        let stream = response_body.write().unwrap();
        stream.blocking_write_and_flush(body).unwrap();
    }

    OutgoingBody::finish(response_body, None).unwrap();
    ResponseOutparam::set(response_out, Ok(response));
}

fn current_timestamp() -> u64 {
    use wasi::clocks::wall_clock::now;
    let duration = now();
    duration.seconds
}
