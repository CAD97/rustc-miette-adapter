# rustc-miette-adapter

It's happening ๐

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

  โ  unused import: `path::PathBuf`
   โญโ[6:1]
 6 โ     fmt::{self, Display},
 7 โ     path::PathBuf,
   ยท     โโโโโโโโโโโโโ
 8 โ     str::FromStr,
   โฐโโโโ

Error:
  โ `#[warn(unused_imports)]` on by default

Error:
  โ remove the unused import
   โญโ[5:1]
 5 โ         error::Error,
 6 โ โญโโถ     fmt::{self, Display},
 7 โ โฐโโถ     path::PathBuf,
 8 โ         str::FromStr,
   โฐโโโโ



  โ  1 warning emitted


unused_imports

  โ  unused import: `WrapErr`
   โญโ[1:1]
 1 โ use miette::{IntoDiagnostic, ReportHandler, Result, WrapErr};
   ยท                                                     โโโโโโโ
 2 โ use rustc_miette_adapter::Diagnostic;
   โฐโโโโ

Error:
  โ `#[warn(unused_imports)]` on by default

Error:
  โ remove the unused import
   โญโ[1:1]
 1 โ use miette::{IntoDiagnostic, ReportHandler, Result, WrapErr};
   ยท                                                   โโโโโโโโโ
 2 โ use rustc_miette_adapter::Diagnostic;
   โฐโโโโ



  โ  1 warning emitted


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
