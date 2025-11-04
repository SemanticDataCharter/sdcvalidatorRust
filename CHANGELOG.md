# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Project Status

This project is currently in the **planning phase** with implementation scheduled for Q2 2026.

### Planned for v4.0.0 (Q2-Q4 2026)

#### Added
- Core XML parsing engine
- XSD 1.0 schema validation
- XSD 1.1 schema validation with assertions
- SDC4 specification compliance
- ExceptionalValue injection (ISO 21090)
- Recovery mode for data preservation
- Error classification system
- Command-line interface
- Library API for Rust applications
- Streaming parser for large documents
- Cross-platform support (Linux, macOS, Windows)
- Comprehensive test suite
- API documentation
- User guides and examples
- Performance benchmarks

## Version History

### [4.0.0-alpha] - 2025-11-04

#### Added
- Initial repository setup
- Project documentation (README.md, CONTRIBUTING.md, SECURITY.md, CLAUDE.md)
- Project structure (src/, tests/, docs/, examples/)
- Cargo.toml configuration
- GitHub workflows for CI/CD
- Issue templates
- publiccode.yml metadata
- MIT License

#### Notes
- This is a planning release with no functional code
- Implementation begins Q2 2026
- Compatible with Rust 1.70+

## Roadmap

### Q2 2026 - Initial Implementation
- [ ] Core XML parsing
- [ ] Basic schema validation
- [ ] Command-line interface
- [ ] Initial test suite

### Q3 2026 - Feature Completion
- [ ] Advanced validation (XSD 1.1, assertions)
- [ ] Recovery mode with ExceptionalValue injection
- [ ] SDC4-specific features
- [ ] Performance optimization

### Q4 2026 - Production Release
- [ ] Comprehensive testing
- [ ] Full documentation
- [ ] Security audit
- [ ] Performance benchmarks
- [ ] v4.0.0 release
- [ ] Publish to crates.io

## Related Projects

- [sdcvalidator (Python)](https://github.com/SemanticDataCharter/sdcvalidator) - Production-ready Python implementation
- [sdcvalidatorJS](https://github.com/SemanticDataCharter/sdcvalidatorJS) - TypeScript/JavaScript implementation
- [SDCRM](https://github.com/SemanticDataCharter/SDCRM) - Reference model and specification

## Support

For questions or issues:
- Open an issue: https://github.com/SemanticDataCharter/sdcvalidatorRust/issues
- Start a discussion: https://github.com/SemanticDataCharter/sdcvalidatorRust/discussions
- Email: support@axius-sdc.com

## Versioning

This project uses semantic versioning:
- **Major** (4.x.x): Breaking changes to API or behavior
- **Minor** (x.Y.x): New features, backwards compatible
- **Patch** (x.x.Z): Bug fixes, backwards compatible

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Legend**:
- `Added` - New features
- `Changed` - Changes to existing functionality
- `Deprecated` - Soon-to-be removed features
- `Removed` - Removed features
- `Fixed` - Bug fixes
- `Security` - Security improvements
