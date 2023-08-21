use crate::services::openai::{
    self, OpenAI, CHAT_COMPLETIONS, IMAGE_GENERATIONS, MODEL_LIST, MODEL_RETRIEVE,
};
use crate::{services, Error, Result};
use axum::extract::Path;
use axum::Json;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    openai: Arc<Mutex<services::openai::OpenAI>>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            openai: OpenAI::get_instance(),
        }
    }
    async fn get_openai(&self) -> tokio::sync::MutexGuard<'_, services::openai::OpenAI> {
        self.openai.lock().await
    }
}

pub fn routes() -> Router {
    let app_state = Arc::new(AppState::new());
    Router::new()
        .route(MODEL_LIST, get(list_models))
        .route(CHAT_COMPLETIONS, post(chat_completion_create))
        .route(MODEL_RETRIEVE, get(retrieve_model))
        .route(IMAGE_GENERATIONS, post(image_create))
        // Add middleware that inserts the state into all incoming request's
        // extensions.
        .layer(Extension(app_state))
}

#[axum::debug_handler]
pub async fn list_models(app_state: Extension<AppState>) -> Result<Json<Value>> {
    let openai = app_state.get_openai().await;
    let models = openai.model_list().await?;
    Ok(Json(models))
}

#[axum::debug_handler]
pub async fn retrieve_model(
    app_state: Extension<AppState>,
    Path(model_id): Path<String>,
) -> Result<Json<Value>> {
    let openai = app_state.get_openai().await;
    let model = openai.model_retrieve(&model_id).await?;
    Ok(Json(model))
}

#[axum::debug_handler]
pub async fn chat_completion_create(
    app_state: Extension<AppState>,
    Json(req): Json<openai::types::ChatCompletionRequest>,
) -> Result<Json<openai::types::ChatCompletionResponse>> {
    let openai = app_state.get_openai().await;
    let response = openai.create_chat_completion(req).await?;
    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn image_create(
    app_state: Extension<AppState>,
    Json(req): Json<openai::types::ImageCreationRequest>,
) -> Result<Json<openai::types::ImageCreationResponse>> {
    let openai = app_state.get_openai().await;
    let response = openai.create_image(req).await?;
    Ok(Json(response))
}
