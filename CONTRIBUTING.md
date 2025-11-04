# Contributing to sdcvalidatorRust

Thank you for your interest in contributing to sdcvalidatorRust! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

## Code of Conduct

This project adheres to a code of conduct that we expect all contributors to follow:

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive feedback
- Assume good intentions
- Prioritize the community and project health

Unacceptable behavior includes harassment, discrimination, trolling, or any conduct that creates an intimidating or hostile environment.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- Git
- A GitHub account

### Finding Ways to Contribute

- Browse [open issues](https://github.com/SemanticDataCharter/sdcvalidatorRust/issues)
- Look for issues tagged `good first issue` or `help wanted`
- Check the [project roadmap](README.md#development-roadmap)
- Improve documentation
- Report bugs or suggest features

## Development Setup

1. **Fork the repository** on GitHub

2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/sdcvalidatorRust.git
   cd sdcvalidatorRust
   ```

3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/SemanticDataCharter/sdcvalidatorRust.git
   ```

4. **Install dependencies and build**:
   ```bash
   cargo build
   ```

5. **Run tests**:
   ```bash
   cargo test
   ```

## Development Workflow

### Creating a Feature Branch

```bash
# Update your main branch
git checkout main
git pull upstream main

# Create a feature branch
git checkout -b feature/your-feature-name
```

### Branch Naming Conventions

- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `refactor/` - Code refactoring
- `test/` - Test additions or modifications
- `chore/` - Maintenance tasks

### Making Changes

1. Make your changes in small, logical commits
2. Write clear commit messages (see below)
3. Add tests for new functionality
4. Update documentation as needed
5. Ensure all tests pass

### Commit Message Format

Follow the conventional commits specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Test changes
- `chore`: Build process or auxiliary tool changes

**Example**:
```
feat(validator): add support for XSD 1.1 assertions

Implement XSD 1.1 assertion validation using the assert element.
This enables more expressive schema constraints.

Closes #123
```

## Coding Standards

### Rust Style Guide

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Write idiomatic Rust code
- Prefer clarity over cleverness

### Code Organization

- Keep modules focused and cohesive
- Use meaningful names for types, functions, and variables
- Document public APIs with doc comments
- Keep functions small and focused
- Minimize dependencies

### Error Handling

- Use `Result` for fallible operations
- Create custom error types when appropriate
- Provide helpful error messages
- Avoid panicking in library code

### Performance Considerations

- Profile before optimizing
- Document performance-critical sections
- Prefer readability unless performance is critical
- Use benchmarks to validate optimizations

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

### Writing Tests

- Write unit tests for individual functions
- Write integration tests for API functionality
- Include edge cases and error conditions
- Use descriptive test names
- Keep tests simple and focused

### Test Coverage

Aim for high test coverage, especially for:
- Core validation logic
- Error handling paths
- Edge cases
- Public API surfaces

## Documentation

### Code Documentation

- Document all public APIs with `///` doc comments
- Include examples in documentation
- Explain complex algorithms or logic
- Document panics, errors, and safety requirements

### Example:
```rust
/// Validates an XML document against an XSD schema.
///
/// # Arguments
///
/// * `xml` - The XML document as a string
/// * `schema` - The XSD schema definition
///
/// # Returns
///
/// Returns a `ValidationResult` containing errors or success status.
///
/// # Examples
///
/// ```
/// use sdcvalidator::Validator;
///
/// let validator = Validator::from_file("schema.xsd")?;
/// let result = validator.validate("<root/>")?;
/// assert!(result.is_valid());
/// ```
///
/// # Errors
///
/// Returns an error if the XML is malformed or the schema is invalid.
pub fn validate(&self, xml: &str) -> Result<ValidationResult, Error> {
    // implementation
}
```

### Project Documentation

- Update README.md for major changes
- Keep CHANGELOG.md current
- Document breaking changes clearly
- Update API documentation

## Submitting Changes

### Before Submitting

1. **Ensure all tests pass**:
   ```bash
   cargo test
   ```

2. **Format your code**:
   ```bash
   cargo fmt
   ```

3. **Check for linting issues**:
   ```bash
   cargo clippy -- -D warnings
   ```

4. **Update documentation**:
   - Update README.md if needed
   - Add entry to CHANGELOG.md
   - Update API docs

5. **Rebase on latest main**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

### Creating a Pull Request

1. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Go to the [repository](https://github.com/SemanticDataCharter/sdcvalidatorRust) on GitHub

3. Click "New Pull Request"

4. Select your branch and create the PR

5. Fill out the PR template with:
   - Clear description of changes
   - Related issue numbers
   - Testing performed
   - Breaking changes (if any)

### Pull Request Review Process

- Maintainers will review your PR
- Address feedback and requested changes
- Keep the PR up to date with main
- Be responsive to comments
- PRs require approval before merging

## Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create release commit
4. Tag release: `git tag v4.x.x`
5. Push tag: `git push upstream v4.x.x`
6. Publish to crates.io: `cargo publish`
7. Create GitHub release with notes

## Communication

- **Issues**: Report bugs and request features
- **Discussions**: Ask questions and discuss ideas
- **Pull Requests**: Submit code changes
- **Email**: security@axius-sdc.com for security issues

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

If you have questions, feel free to:
- Open an issue with the `question` label
- Start a discussion on GitHub
- Reach out to maintainers

Thank you for contributing to sdcvalidatorRust!
