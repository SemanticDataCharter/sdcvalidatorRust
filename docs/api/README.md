# API Reference

Complete API documentation for sdcvalidatorRust.

## Status

API documentation will be available with the initial release in Q2 2026.

## Planned API

The API is designed to be simple and intuitive:

### Core Types

- `Validator` - Main validation interface
- `ValidationOptions` - Configuration for validation
- `ValidationResult` - Results of validation
- `RecoveryOptions` - Configuration for recovery mode
- `ExceptionalValueType` - ISO 21090 exceptional value types

### Main Functions

- `Validator::from_file()` - Create validator from schema file
- `Validator::validate()` - Validate XML document
- `Validator::validate_with_recovery()` - Validate and recover invalid data

## Full Documentation

Once implemented, full API documentation will be available at:
- [docs.rs/sdcvalidator](https://docs.rs/sdcvalidator)
- Inline documentation via `cargo doc --open`

## Examples

See the [examples directory](../../examples/) for usage examples.
