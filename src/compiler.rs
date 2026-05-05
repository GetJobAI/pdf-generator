use tracing::warn;
use typst::{diag::Warned, layout::PagedDocument};
use typst_as_lib::{TypstAsLibError, TypstEngine};
use typst_pdf::PdfOptions;

use std::sync::Arc;

use crate::error::Error;
use crate::resume::ResumeData;
use crate::typst_writer;

// Embedded at compile time. Path is relative to this source file (src/).
static TEMPLATE: &str = include_str!("../template.typ");

static JBM_REGULAR: &[u8] = include_bytes!("../fonts/JetBrainsMono-Regular.ttf");
static JBM_BOLD: &[u8] = include_bytes!("../fonts/JetBrainsMono-Bold.ttf");

/// Shared Typst compiler. `Clone` is cheap.
#[derive(Clone)]
pub struct Compiler {
    fonts: Arc<Vec<&'static [u8]>>,
}

impl Compiler {
    pub fn new() -> Self {
        let fonts = Arc::new(
            typst_assets::fonts()
                .chain([JBM_REGULAR, JBM_BOLD])
                .collect(),
        );

        Self { fonts }
    }

    /// Serialises `data` to a Typst source file, compiles it in-memory, and
    /// returns the PDF bytes. No temp files or disk I/O.
    pub fn compile(&self, data: &ResumeData) -> Result<Vec<u8>, Error> {
        let entrypoint = typst_writer::render(data);

        let engine = TypstEngine::builder()
            .fonts(self.fonts.iter().copied())
            .with_static_source_file_resolver([
                ("template.typ", TEMPLATE),
                ("main.typ", entrypoint.as_str()),
            ])
            .build();

        let result: Warned<Result<PagedDocument, _>> = engine.compile("main.typ");

        for warning in &result.warnings {
            warn!("Typst warning: {}", warning.message);
        }

        let document = result.output.map_err(|err| {
            let diagnostics = extract_diagnostics(&err);
            Error::Compile {
                message: err.to_string(),
                diagnostics,
            }
        })?;

        let pdf_bytes = typst_pdf::pdf(&document, &PdfOptions::default()).map_err(|diags| {
            let msg = diags
                .iter()
                .map(|d| d.message.to_string())
                .collect::<Vec<_>>()
                .join("; ");
            Error::PdfExport(msg)
        })?;

        Ok(pdf_bytes)
    }
}

fn extract_diagnostics(err: &TypstAsLibError) -> Vec<String> {
    match err {
        TypstAsLibError::TypstSource(diags) => {
            diags.iter().map(|d| d.message.to_string()).collect()
        }
        _ => Vec::new(),
    }
}
