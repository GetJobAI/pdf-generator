use axum::{
    Json, Router,
    body::Bytes,
    extract::State,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_scalar::{Scalar, Servable};

use crate::compiler::Compiler;
use crate::error::Error;
use crate::resume::{ResumeData, resume_example};

#[derive(OpenApi)]
#[openapi(info(
    title = "PDF Generator",
    version = env!("CARGO_PKG_VERSION"),
    description = "Compiles resume JSON to PDF using the GetJobAI Typst template."
))]
struct ApiDoc;

/// Error response returned on compilation failure or internal error.
#[derive(Serialize, ToSchema)]
struct ErrorResponse {
    /// Human-readable error description.
    error: String,
    /// Typst compiler diagnostics, if available.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    diagnostics: Vec<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub compiler: Compiler,
}

struct HttpError(Error);

impl From<Error> for HttpError {
    fn from(e: Error) -> Self {
        Self(e)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, body) = match self.0 {
            Error::Compile {
                ref message,
                ref diagnostics,
            } => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: message.clone(),
                    diagnostics: diagnostics.clone(),
                },
            ),
            ref e => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: e.to_string(),
                    diagnostics: vec![],
                },
            ),
        };
        (status, Json(body)).into_response()
    }
}

/// Compile resume JSON to a PDF file.
///
/// Some text strings support markdown with the following format:
/// `**bold**`, `*bold*`, `_italic_`, `` `code` ``.
/// Markers may be nested (e.g. `_**bold italic**_`). Unmatched markers are
/// treated as plain text. Injection prevention mechanisms are in place.
#[utoipa::path(
    post,
    path = "/generate",
    tag = "PDF",
    request_body(
        content = ResumeData,
        content_type = "application/json",
        examples(("professional" = (
            summary = "Professional style — Jane Doe",
            value = json!(resume_example())
        )))
    ),
    responses(
        (status = 200, description = "Compiled PDF", content_type = "application/pdf"),
        (status = 400, description = "Typst compilation failed", body = ErrorResponse),
        (status = 422, description = "Request body is not valid JSON or fails schema validation"),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    )
)]
async fn generate_pdf(
    State(state): State<AppState>,
    Json(data): Json<ResumeData>,
) -> Result<impl IntoResponse, HttpError> {
    let compiler = state.compiler.clone();

    let pdf_bytes = tokio::task::spawn_blocking(move || compiler.compile(&data))
        .await
        .map_err(|e| HttpError(Error::PdfExport(e.to_string())))??;

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/pdf")],
        Bytes::from(pdf_bytes),
    ))
}

/// Health check.
#[utoipa::path(
    get,
    path = "/health",
    tag = "Meta",
    responses((status = 200, description = "Service is healthy", body = str))
)]
async fn health() -> &'static str {
    "ok"
}

pub fn router(compiler: Compiler) -> Router {
    let state = AppState { compiler };

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(generate_pdf))
        .routes(routes!(health))
        .with_state(state)
        .split_for_parts();

    let spec = api.clone();

    router
        .route("/openapi.json", get(async || Json(spec)))
        .merge(Scalar::with_url("/docs", api))
}
