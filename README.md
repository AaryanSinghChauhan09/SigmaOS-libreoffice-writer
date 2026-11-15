# SigmaOS-libreoffice-writer

> **LibreOffice Writer — Word Processor** — Full-featured word processor and document layout (LibreOffice Writer parity).

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) ecosystem.  
Parity / inspiration: **LibreOffice**

## Quickstart

```bash
cargo test    # run deterministic AAA unit tests
cargo check   # verify compilation
```

## Component Role

This crate is a component of the SigmaOS LibreOffice Writer suite:
- Fully real Rust implementation (no stubs, no placeholders)
- Fail-fast validation with typed domain errors
- Deterministic unit tests (Arrange-Act-Assert pattern)
- Single Responsibility, Modular, Loose Coupling

## API

```rust
use sigmaos_libreoffice_writer::*;
// See src/lib.rs for the full public API surface
```

## Documentation

Full documentation: [SigmaOS GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

## License

LGPL-2.1
