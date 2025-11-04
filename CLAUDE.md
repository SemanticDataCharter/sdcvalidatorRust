# Developer Guide and Architecture Documentation

This document provides detailed information for developers and AI assistants working on sdcvalidatorRust.

## Project Overview

**sdcvalidatorRust** is a high-performance XML Schema validator implementing the Semantic Data Charter (SDC4) specification. It's designed to provide fast, memory-efficient validation with a unique "quarantine-and-tag" recovery mode that preserves invalid data rather than rejecting it.

### Key Goals

1. **Performance**: Leverage Rust's zero-cost abstractions for optimal speed
2. **Memory Efficiency**: Streaming parsing and validation where possible
3. **SDC4 Compliance**: Full implementation of SDC4 specification
4. **Correctness**: Comprehensive testing and formal verification
5. **Usability**: Intuitive API for both CLI and library usage

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     CLI Interface                            │
│                    (src/cli/mod.rs)                          │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                  Public API Layer                            │
│            (src/lib.rs, src/api/mod.rs)                      │
└─────────────────────┬───────────────────────────────────────┘
                      │
        ┌─────────────┼─────────────┐
        │             │             │
┌───────▼─────┐  ┌───▼─────┐  ┌───▼──────────┐
│   Parser    │  │ Schema  │  │   Recovery   │
│   Engine    │  │ Engine  │  │    Engine    │
│             │  │         │  │              │
│ src/parser/ │  │src/     │  │src/recovery/ │
│             │  │schema/  │  │              │
└─────────────┘  └─────────┘  └──────────────┘
        │             │             │
        └─────────────┼─────────────┘
                      │
        ┌─────────────┴─────────────┐
        │                           │
┌───────▼──────┐          ┌─────────▼────────┐
│    Error     │          │  ExceptionalValue│
│  Classifier  │          │     Injector     │
│              │          │                  │
│src/errors/   │          │src/recovery/ev/  │
└──────────────┘          └──────────────────┘
```

### Core Components

#### 1. Parser Engine (`src/parser/`)

Responsible for parsing XML documents and schemas.

**Modules**:
- `xml.rs` - XML document parsing
- `xsd.rs` - XSD schema parsing
- `streaming.rs` - Streaming parser for large documents
- `types.rs` - XML type definitions

**Key Design Decisions**:
- Use streaming parser for memory efficiency
- Support both DOM-style and SAX-style parsing
- Handle XML namespaces correctly
- Preserve document structure for recovery mode

#### 2. Schema Engine (`src/schema/`)

Validates XML documents against XSD schemas.

**Modules**:
- `validator.rs` - Main validation logic
- `xsd10.rs` - XSD 1.0 validation
- `xsd11.rs` - XSD 1.1 validation (assertions, etc.)
- `datatypes.rs` - Built-in XML Schema datatypes
- `sdc4.rs` - SDC4-specific validation extensions

**Key Design Decisions**:
- Compile schemas once, validate many documents
- Support both XSD 1.0 and 1.1
- Efficient error collection
- Extensible for SDC4 features

#### 3. Recovery Engine (`src/recovery/`)

Implements the "quarantine-and-tag" pattern.

**Modules**:
- `mod.rs` - Recovery orchestration
- `ev.rs` - ExceptionalValue injection
- `classifier.rs` - Error to EV type mapping
- `xml_rewriter.rs` - XML document modification

**Key Design Decisions**:
- Preserve original data structure
- Insert ExceptionalValue elements at appropriate locations
- Map validation errors to ISO 21090 EV types
- Maintain document validity after recovery

#### 4. Error Classifier (`src/errors/`)

Maps validation errors to appropriate ExceptionalValue types.

**Modules**:
- `types.rs` - Error type definitions
- `classifier.rs` - Classification logic
- `mapping.rs` - Error to EV type mapping rules

**Key Design Decisions**:
- Use pattern matching for error classification
- Support custom classification rules
- Provide detailed error information
- Enable error aggregation

### Data Flow

#### Standard Validation

```
XML Document → Parser → Schema Validator → Validation Result
                  ↓           ↓
              XML Tree    Schema Model
```

#### Recovery Mode

```
XML Document → Parser → Schema Validator → Error Classifier
                  ↓           ↓                  ↓
              XML Tree    Schema Model    ExceptionalValue Types
                  ↓                              ↓
                  └──────→ Recovery Engine ──────┘
                              ↓
                        Modified XML Document
```

## Implementation Plan

### Phase 1: Core Infrastructure (Q2 2026)

**Week 1-2: Project Setup**
- Set up Cargo workspace
- Configure CI/CD
- Set up testing framework
- Define core types and traits

**Week 3-4: XML Parser**
- Implement basic XML parsing
- Add namespace support
- Add streaming parser
- Write parser tests

**Week 5-6: Schema Parser**
- Parse XSD schemas
- Build schema model
- Handle includes/imports
- Write schema parser tests

**Week 7-8: Basic Validation**
- Implement element validation
- Implement type validation
- Add error reporting
- Write validation tests

### Phase 2: Feature Completion (Q3 2026)

**Week 1-2: Advanced Validation**
- XSD 1.1 assertions
- Complex type derivation
- Schema component constraints
- Identity constraints

**Week 3-4: Recovery Engine**
- Error classification
- ExceptionalValue injection
- XML rewriting
- Recovery tests

**Week 5-6: SDC4 Features**
- SDC4-specific validation rules
- Custom ExceptionalValue handling
- SDC4 metadata preservation

**Week 7-8: CLI Interface**
- Command-line argument parsing
- File I/O
- Progress reporting
- Error formatting

### Phase 3: Production Ready (Q4 2026)

**Week 1-2: Performance Optimization**
- Profiling and benchmarking
- Optimize hot paths
- Reduce allocations
- Parallel validation

**Week 3-4: Documentation**
- API documentation
- User guides
- Examples
- Architecture docs

**Week 5-6: Testing & QA**
- Comprehensive test suite
- Integration tests
- Fuzzing
- Security audit

**Week 7-8: Release Preparation**
- Package for crates.io
- Release notes
- Migration guide
- Version 1.0 release

## Code Style and Conventions

### Naming Conventions

- **Modules**: `snake_case` (e.g., `xml_parser`)
- **Types**: `PascalCase` (e.g., `ValidationResult`)
- **Functions**: `snake_case` (e.g., `validate_element`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_DEPTH`)
- **Lifetimes**: Single lowercase letter (e.g., `'a`, `'input`)

### Module Organization

```rust
// src/validator.rs

//! Module-level documentation
//!
//! Describes the purpose and usage of this module.

use std::collections::HashMap;
use crate::schema::Schema;

// Private constants
const DEFAULT_MAX_DEPTH: usize = 100;

// Public types
pub struct Validator {
    schema: Schema,
    options: ValidationOptions,
}

// Public implementation
impl Validator {
    /// Creates a new validator from a schema file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the XSD schema file
    ///
    /// # Errors
    ///
    /// Returns an error if the schema file cannot be read or parsed.
    pub fn from_file(path: &str) -> Result<Self, Error> {
        // Implementation
    }
}

// Private implementation
impl Validator {
    fn validate_internal(&self, xml: &str) -> Result<(), Error> {
        // Implementation
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        // Test implementation
    }
}
```

### Error Handling

Use custom error types with `thiserror`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidatorError {
    #[error("Failed to parse XML: {0}")]
    XmlParseError(String),

    #[error("Schema validation failed: {0}")]
    ValidationError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ValidatorError>;
```

### Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Unit tests - test individual functions
    #[test]
    fn test_parse_simple_xml() {
        let xml = "<root/>";
        let result = parse_xml(xml);
        assert!(result.is_ok());
    }

    // Integration tests - test components together
    #[test]
    fn test_validation_workflow() {
        let schema = Schema::from_file("test.xsd").unwrap();
        let validator = Validator::new(schema);
        let result = validator.validate("<root/>");
        assert!(result.is_valid());
    }

    // Edge case tests
    #[test]
    fn test_empty_document() {
        let result = parse_xml("");
        assert!(result.is_err());
    }
}
```

## Dependencies

### Core Dependencies

- **quick-xml**: Fast XML parsing
- **roxmltree**: XML tree representation
- **thiserror**: Error handling
- **anyhow**: Error context
- **clap**: CLI argument parsing

### Development Dependencies

- **criterion**: Benchmarking
- **proptest**: Property-based testing
- **pretty_assertions**: Better test output

### Minimizing Dependencies

- Prefer standard library when possible
- Evaluate dependency security and maintenance
- Document why each dependency is needed
- Keep dependency tree shallow

## Testing

### Test Categories

1. **Unit Tests**: Test individual functions and methods
2. **Integration Tests**: Test component interactions
3. **Property Tests**: Test invariants with random inputs
4. **Benchmark Tests**: Measure performance
5. **Example Tests**: Verify examples compile and run

### Test Data

Store test files in `tests/fixtures/`:
```
tests/
├── fixtures/
│   ├── schemas/
│   │   ├── simple.xsd
│   │   ├── complex.xsd
│   │   └── sdc4.xsd
│   └── documents/
│       ├── valid/
│       │   ├── simple.xml
│       │   └── complex.xml
│       └── invalid/
│           ├── type_error.xml
│           └── structure_error.xml
├── integration_tests.rs
└── validation_tests.rs
```

## Performance Considerations

### Optimization Targets

1. **Parsing**: < 100ms for typical documents (< 1MB)
2. **Validation**: < 10ms overhead over parsing
3. **Recovery**: < 50ms for typical error injection
4. **Memory**: < 10x document size

### Profiling

```bash
# CPU profiling
cargo build --release
CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --bin sdcvalidator

# Memory profiling
valgrind --tool=massif target/release/sdcvalidator

# Benchmarking
cargo bench
```

## Security Considerations

### Input Validation

- Validate all external input
- Set resource limits (memory, depth, time)
- Disable dangerous features by default (external entities)
- Sanitize error messages

### Fuzzing

```bash
# Install cargo-fuzz
cargo install cargo-fuzz

# Run fuzzer
cargo fuzz run parser_fuzzer
```

## Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Security audit completed
- [ ] Performance benchmarks run
- [ ] Examples tested
- [ ] Release notes written
- [ ] Git tag created
- [ ] Published to crates.io
- [ ] GitHub release created

## Resources

### Standards

- [XML 1.0 Specification](https://www.w3.org/TR/xml/)
- [XML Schema 1.0](https://www.w3.org/TR/xmlschema-1/)
- [XML Schema 1.1](https://www.w3.org/TR/xmlschema11-1/)
- [ISO 21090](https://www.iso.org/standard/35646.html)

### Related Projects

- [sdcvalidator](https://github.com/SemanticDataCharter/sdcvalidator) - Python implementation
- [sdcvalidatorJS](https://github.com/SemanticDataCharter/sdcvalidatorJS) - TypeScript implementation
- [SDCRM](https://github.com/SemanticDataCharter/SDCRM) - Reference model

### Rust Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)

## AI Assistant Guidelines

When working on this project:

1. **Follow the architecture**: Maintain separation of concerns
2. **Write tests first**: TDD approach preferred
3. **Document as you go**: Update docs with code changes
4. **Consider performance**: Profile before optimizing
5. **Security first**: Validate inputs, handle errors safely
6. **Keep it simple**: Prefer clarity over cleverness
7. **Ask questions**: Clarify requirements before implementing

### Common Tasks

**Adding a new validation rule**:
1. Add test case in `tests/validation_tests.rs`
2. Implement rule in `src/schema/validator.rs`
3. Update error classifier if needed
4. Document in API docs

**Adding a new ExceptionalValue type**:
1. Add variant to `ExceptionalValueType` enum
2. Update classifier mapping
3. Add injection logic
4. Write tests

**Performance optimization**:
1. Profile current performance
2. Identify bottleneck
3. Implement optimization
4. Benchmark to verify improvement
5. Ensure tests still pass

## Contact

For questions or clarifications:
- Open a GitHub discussion
- Check existing documentation
- Review similar code in Python/TypeScript implementations
- Ask maintainers for guidance

---

This document is a living guide. Update it as the architecture evolves!
