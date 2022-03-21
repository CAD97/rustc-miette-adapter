// TODO: set up a SourceMap so SourceSpan can index multiple files 😅

use miette::{LabeledSpan, Severity};
use std::{
    error::Error,
    fmt::{self, Display},
    path::PathBuf,
    str::FromStr,
};

#[derive(Debug)]
pub struct Diagnostic {
    message: String,
    source_code: Option<String>,
    code: Option<String>,
    severity: Option<Severity>,
    labels: Vec<LabeledSpan>,
    related: Vec<Diagnostic>,
}

impl Error for Diagnostic {}
impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl miette::Diagnostic for Diagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        let code = self.code.as_deref()?;
        Some(Box::new(code))
    }

    fn severity(&self) -> Option<Severity> {
        self.severity
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        let source = self.source_code.as_ref()?;
        Some(source)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        Some(Box::new(self.labels.iter().cloned()))
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn miette::Diagnostic> + 'a>> {
        Some(Box::new(
            self.related
                .iter()
                .map(|x| x as &(dyn 'a + miette::Diagnostic)),
        ))
    }
}

impl From<cargo::diagnostic::Diagnostic> for Diagnostic {
    fn from(d: cargo::diagnostic::Diagnostic) -> Self {
        Diagnostic {
            message: d.message,
            source_code: d
                .spans
                .iter()
                .next()
                .and_then(|s| std::fs::read_to_string(&s.file_name).ok()),
            code: d.code.map(|c| c.code),
            severity: match d.level {
                cargo::diagnostic::DiagnosticLevel::Ice
                | cargo::diagnostic::DiagnosticLevel::Error => Some(Severity::Error),
                cargo::diagnostic::DiagnosticLevel::Warning => Some(Severity::Warning),
                cargo::diagnostic::DiagnosticLevel::FailureNote
                | cargo::diagnostic::DiagnosticLevel::Note
                | cargo::diagnostic::DiagnosticLevel::Help => Some(Severity::Advice),
                _ => None,
            },
            labels: d
                .spans
                .into_iter()
                .map(|s| {
                    LabeledSpan::new_with_span(s.label, s.byte_start as usize..s.byte_end as usize)
                })
                .collect(),
            related: d.children.into_iter().map(Into::into).collect(),
        }
    }
}

impl FromStr for Diagnostic {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str::<cargo::diagnostic::Diagnostic>(s).map(Into::into)
    }
}
