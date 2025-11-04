# Getting Started

This guide will help you get started with sdcvalidatorRust.

## Status

This project is currently in the planning phase with implementation scheduled for Q2 2026.

For production use, please see:
- [sdcvalidator (Python)](https://github.com/SemanticDataCharter/sdcvalidator)
- [sdcvalidatorJS (TypeScript/JavaScript)](https://github.com/SemanticDataCharter/sdcvalidatorJS)

## Planned Installation

Once released, installation will be available via Cargo:

```bash
cargo install sdcvalidator
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
sdcvalidator = "4.0"
```

## Quick Start Example

The following example shows the planned API:

```rust
use sdcvalidator::{Validator, ValidationOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create validator with schema
    let validator = Validator::from_file("schema.xsd")?;

    // Validate XML document
    let xml = std::fs::read_to_string("data.xml")?;
    let result = validator.validate(&xml)?;

    if result.is_valid() {
        println!("Document is valid!");
    } else {
        println!("Validation errors: {:?}", result.errors());
    }

    Ok(())
}
```

## Coming Soon

Full documentation will be available with the initial release. Stay tuned!
