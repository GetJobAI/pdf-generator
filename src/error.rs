#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Typst compilation failed: {message}")]
    Compile {
        message: String,
        diagnostics: Vec<String>,
    },

    #[error("PDF export failed: {0}")]
    PdfExport(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
