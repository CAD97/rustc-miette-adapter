# rustc-miette-adapter

It's happening 👐

```
> cargo check --quiet
warning: unused import: `path::PathBuf`
 --> src\lib.rs:7:5
  |
7 |     path::PathBuf,
  |     ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `WrapErr`
 --> src\main.rs:1:53
  |
1 | use miette::{IntoDiagnostic, ReportHandler, Result, WrapErr};
  |                                                     ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

> cargo check --message-format=json --quiet | .\target\debug\rustc-miette-adapter.exe
unused_imports

  ⚠ unused import: `path::PathBuf`
   ╭─[6:1]
 6 │     fmt::{self, Display},
 7 │     path::PathBuf,
   ·     ─────────────
 8 │     str::FromStr,
   ╰────

Error:
  ☞ `#[warn(unused_imports)]` on by default

Error:
  ☞ remove the unused import
   ╭─[5:1]
 5 │         error::Error,
 6 │ ╭─▶     fmt::{self, Display},
 7 │ ╰─▶     path::PathBuf,
 8 │         str::FromStr,
   ╰────



  ⚠ 1 warning emitted


unused_imports

  ⚠ unused import: `WrapErr`
   ╭─[1:1]
 1 │ use miette::{IntoDiagnostic, ReportHandler, Result, WrapErr};
   ·                                                     ───────
 2 │ use rustc_miette_adapter::Diagnostic;
   ╰────

Error:
  ☞ `#[warn(unused_imports)]` on by default

Error:
  ☞ remove the unused import
   ╭─[1:1]
 1 │ use miette::{IntoDiagnostic, ReportHandler, Result, WrapErr};
   ·                                                   ─────────
 2 │ use rustc_miette_adapter::Diagnostic;
   ╰────



  ⚠ 1 warning emitted


> 
```

![image](https://user-images.githubusercontent.com/5992217/159197762-bdb7b80d-62ac-48c5-a5b4-c92f3aec756b.png)

![image](https://user-images.githubusercontent.com/5992217/159197778-f2a1fa63-1a77-45c6-ab0c-5b1fd2220d38.png)

## Motivation

Duh.

## Limitations

Many. Mainly,

- Does a naive translation to miette diagnostics; does not try to be smart about how the mapping is done
- Cannot handle (single) diagnostics with spans from multiple files
