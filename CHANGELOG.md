# Changelog

All notable changes to the Offline Doctor AI project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial application setup with Tauri + React
- Offline AI inference using llama.cpp integration
- Medical-focused chat interface
- Model download and management system
- Conversation history with sled database
- Support for multiple medical AI models:
  - Llama 3.2 3B Instruct (Q4)
  - Llama 3.1 8B Instruct (Q4) 
  - OpenBioLLM 8B (Q4)
- Cross-platform builds (Linux, Windows, macOS)
- GitHub Actions workflows for CI/CD

### Technical
- React + TypeScript frontend with Tailwind CSS
- Rust backend with Tauri framework
- Sled embedded database for conversation storage
- Thread-safe async architecture
- Medical prompt templates and disclaimers

## [0.1.0] - 2025-06-17

### Added
- Initial release of Offline Doctor AI
- Basic chat functionality
- Model management interface
- Offline AI inference capabilities

---

## Release Notes Template

When creating a new release, use this template:

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security improvements

### Medical
- Medical accuracy improvements
- New medical models
- Updated medical disclaimers
```
