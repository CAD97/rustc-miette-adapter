use miette::{IntoDiagnostic, ReportHandler, Result, WrapErr};
use rustc_miette_adapter::Diagnostic;
use std::{fmt, io::prelude::*};

struct DisplayDiagnostic<'a>(&'a dyn miette::Diagnostic);

impl fmt::Display for DisplayDiagnostic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        miette::GraphicalReportHandler::new().debug(self.0, f)
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    for message in cargo::Message::parse_stream(stdin) {
        if let cargo::Message::CompilerMessage(m) = message.into_diagnostic()? {
            let d: Diagnostic = m.message.into();
            writeln!(stdout, "{}", DisplayDiagnostic(&d)).into_diagnostic()?
        }
    }

    Ok(())
}
