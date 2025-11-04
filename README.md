# sdc4-validator

A high-performance XML Schema validator with Semantic Data Charter (SDC4) support, implemented in Rust.

> **Developed and maintained by [Axius SDC, Inc.](https://axius-sdc.com)** in support of the Semantic Data Charter community.

## Overview

**sdc4-validator** is a Rust implementation of the SDC4 validation framework, providing fast and memory-efficient validation of XML documents against XML Schema definitions with specialized support for healthcare data quality management through the Semantic Data Charter specification.

Unlike traditional validators that reject invalid data, sdc4-validator implements a "quarantine-and-tag" pattern that preserves problematic data while injecting ISO 21090-based `ExceptionalValue` elements. This approach enables comprehensive data quality assessment without data loss.

## Status

⚠️ **PLACEHOLDER RELEASE (v0.1.0) - NOT FUNCTIONAL**

This package has been published to crates.io to reserve the name. The full implementation is scheduled for **Q2 2026** and will be released as **v4.0.0**.

This repository serves as a placeholder and design specification for the future Rust implementation. For production use today, please see:
- [sdcvalidator (Python)](https://github.com/SemanticDataCharter/sdcvalidator) - Production-ready Python implementation
- [sdcvalidatorJS (TypeScript/JavaScript)](https://github.com/SemanticDataCharter/sdcvalidatorJS) - npm package for Node.js and browsers

## Features (Planned)

- **High Performance**: Native Rust implementation for optimal speed and memory efficiency
- **SDC4 Compliance**: Full support for Semantic Data Charter 4.x specification
- **ExceptionalValue Support**: Automatic injection of ISO 21090-based exceptional values
- **Recovery Mode**: Tag and preserve invalid data rather than rejecting it
- **CLI Interface**: Command-line tool for batch validation
- **Library API**: Embeddable validation engine for Rust applications
- **Zero Dependencies**: Minimal dependency footprint for easy integration
- **Cross-Platform**: Support for Linux, macOS, and Windows

## Installation (Coming Q2 2026)

The package is published on crates.io as a placeholder. **Do not use v0.1.x in production.**

Once the full implementation is released (v4.0.0), installation will be:

```bash
cargo install sdc4-validator
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
sdc4-validator = "4.0"
```

## Planned Usage

### Command Line Interface

```bash
# Validate an XML file against a schema
sdc4-validate --schema schema.xsd --input data.xml

# Enable recovery mode (inject ExceptionalValues)
sdc4-validate --schema schema.xsd --input data.xml --recovery

# Output validation report
sdc4-validate --schema schema.xsd --input data.xml --output report.json
```

### Library Usage

```rust
use sdc4_validator::{Validator, ValidationOptions};

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

### Recovery Mode

```rust
use sdc4_validator::{Validator, RecoveryOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let validator = Validator::from_file("schema.xsd")?;
    let xml = std::fs::read_to_string("data.xml")?;

    // Enable recovery mode to inject ExceptionalValues
    let options = RecoveryOptions::default();
    let result = validator.validate_with_recovery(&xml, options)?;

    // Get modified XML with ExceptionalValues injected
    let recovered_xml = result.recovered_xml();
    std::fs::write("data_recovered.xml", recovered_xml)?;

    Ok(())
}
```

## Architecture

The validator will be built on these core components:

- **XML Parser**: Fast XML parsing with streaming support
- **Schema Engine**: XSD 1.0/1.1 validation with SDC4 extensions
- **Recovery Engine**: ExceptionalValue injection and data preservation
- **Error Classifier**: Maps validation errors to ISO 21090 exceptional value types
- **CLI Interface**: User-friendly command-line tool

## ExceptionalValue Types

The validator will support all ISO 21090 exceptional value types:

| Type | Code | Description |
|------|------|-------------|
| No Information | `NI` | No information available |
| Masked | `MSK` | Information present but masked for privacy |
| Not Applicable | `NA` | Not applicable in this context |
| Unknown | `UNK` | Information was sought but not found |
| Asked But Unknown | `ASKU` | Information was asked but is unknown |
| Temporarily Unavailable | `NAV` | Temporarily unavailable |
| Not Asked | `NASK` | Not asked or assessed |
| Sufficient Quantity | `QS` | Sufficient quantity recorded |
| Trace | `TRC` | Trace amounts detected |
| Negative Infinity | `NINF` | Negative infinity |
| Positive Infinity | `PINF` | Positive infinity |
| Other | `OTH` | Other reason |

## Development Roadmap

- **Q2 2026**: Initial implementation
  - Core XML parsing and validation
  - Basic SDC4 support
  - CLI interface
- **Q3 2026**: Feature completion
  - Full ExceptionalValue support
  - Recovery mode
  - Performance optimization
- **Q4 2026**: Production release
  - Comprehensive testing
  - Documentation
  - Package publication

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to participate in this project.

## Security

For security concerns or vulnerability reports, please see our [Security Policy](SECURITY.md).

## Documentation

- [Getting Started Guide](docs/getting-started.md) (Coming soon)
- [API Reference](docs/api/README.md) (Coming soon)
- [Developer Guide](CLAUDE.md)
- [SDC4 Specification](https://github.com/SemanticDataCharter/SDCRM)

## Related Projects

This validator is part of the Semantic Data Charter ecosystem:

- [SDCRM](https://github.com/SemanticDataCharter/SDCRM) - Reference model and specification
- [sdcvalidator (Python)](https://github.com/SemanticDataCharter/sdcvalidator) - Reference implementation
- [@semanticdatacharter/validator (JavaScript/TypeScript)](https://github.com/SemanticDataCharter/sdcvalidatorJS) - npm package
- [sdc4-validator (Rust)](https://github.com/SemanticDataCharter/sdcvalidatorRust) - This library (planned Q2 2026)
- [SDC Obsidian Template](https://github.com/SemanticDataCharter/SDCObsidianTemplate) - Markdown templates

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Trademarks

"Semantic Data Charter" and "SDC4" are trademarks of Axius SDC, Inc.

## Support

- Issues: [GitHub Issues](https://github.com/SemanticDataCharter/sdcvalidatorRust/issues)
- Discussions: [GitHub Discussions](https://github.com/SemanticDataCharter/sdcvalidatorRust/discussions)
- Email: support@axius-sdc.com

## Sponsors

This project is developed and maintained by **[Axius SDC, Inc.](https://axius-sdc.com)** in support of the Semantic Data Charter community.

### Trademarks

"Semantic Data Charter" and "SDC4" are trademarks of Axius SDC, Inc.

## Acknowledgments

Built upon 15+ years of research and development in healthcare data standards and semantic validation. Implements concepts from international standards including W3C XML Schema, ISO 21090, HL7, and ODM.
