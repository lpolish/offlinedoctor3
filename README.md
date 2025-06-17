# Offline Doctor AI

[![CI](https://github.com/lpolish/offlinedoctor3/workflows/CI/badge.svg)](https://github.com/lpolish/offlinedoctor3/actions/workflows/ci.yml)
[![Release](https://github.com/lpolish/offlinedoctor3/workflows/Release/badge.svg)](https://github.com/lpolish/offlinedoctor3/actions/workflows/release.yml)
[![Security](https://img.shields.io/badge/security-policy-blue)](SECURITY.md)

A completely offline medical AI assistant designed for healthcare professionals, particularly resident doctors in remote locations. This application provides genuine AI-powered medical consultations without requiring an internet connection.

## 🎯 Key Features

- **100% Offline Operation**: No internet required after initial setup
- **Real AI Inference**: Uses llama.cpp with actual language models (no mocked responses)
- **Medical Focus**: Specialized prompting for healthcare scenarios
- **Conversation Management**: Persistent chat history with SQLite storage
- **Model Selection**: Support for multiple AI models of different sizes
- **Cross-Platform**: Built with Tauri for Windows, macOS, and Linux

## 🏗️ Architecture

### Frontend
- **React + TypeScript**: Modern, type-safe UI development
- **Tailwind CSS**: Responsive, medical-focused interface design
- **Lucide Icons**: Clean, professional iconography

### Backend
- **Rust + Tauri**: High-performance, secure native application
- **llama.cpp Integration**: Local AI inference via native binary
- **SQLite Database**: Lightweight, embedded conversation storage
- **Model Management**: Automatic download and version handling

### AI Engine
- **Local Inference**: Uses downloaded GGUF format models
- **Medical Prompting**: Contextual prompts for healthcare scenarios
- **Conversation Context**: Maintains chat history for better responses
- **Resource Efficient**: Optimized for various hardware configurations

## 🚀 Getting Started

### Prerequisites
- Node.js (v18 or higher)
- Rust (latest stable)
- llama.cpp binary (for AI inference)

### Installation

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd offlinedoctor3
   ```

2. **Install dependencies**:
   ```bash
   npm install
   ```

3. **Install llama.cpp** (required for AI functionality):
   - **Option A**: Install via package manager
     ```bash
     # On Ubuntu/Debian
     sudo apt install llama.cpp
     
     # On macOS
     brew install llama.cpp
     ```
   
   - **Option B**: Build from source
     ```bash
     git clone https://github.com/ggerganov/llama.cpp
     cd llama.cpp
     make llama-server
     sudo cp llama-server /usr/local/bin/
     ```

4. **Run the development server**:
   ```bash
   npm run tauri dev
   ```

### First Time Setup

1. **Launch the application** - it will show the setup screen
2. **Download a model** - choose from available medical-focused models:
   - **Llama 3.2 3B** (~2.1GB) - Compact, good for basic queries
   - **Llama 3.1 8B** (~4.7GB) - Higher quality, better reasoning
   - **OpenBioLLM 8B** (~4.8GB) - Medical-specific training
3. **Initialize AI Engine** - select your downloaded model
4. **Start chatting** - ask medical questions and get AI-powered responses

## 📦 Installation & Releases

### Pre-built Binaries

Download the latest release for your platform:

- **Linux**: `.deb`, `.rpm`, or `.AppImage` packages
- **Windows**: `.msi` installer
- **macOS**: `.app` bundle (Intel and Apple Silicon)

Visit the [Releases](../../releases) page to download the latest version.

### Building from Source

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd offlinedoctor3
   ```

2. **Install dependencies**:
   ```bash
   npm install
   ```

3. **Install llama.cpp** (required for AI functionality):
   - **Option A**: Install via package manager
     ```bash
     # On Ubuntu/Debian
     sudo apt install llama.cpp
     
     # On macOS
     brew install llama.cpp
     ```
   
   - **Option B**: Build from source
     ```bash
     git clone https://github.com/ggerganov/llama.cpp
     cd llama.cpp
     make llama-server
     sudo cp llama-server /usr/local/bin/
     ```

4. **Run the development server**:
   ```bash
   npm run tauri dev
   ```

### 🔄 Development Workflows

This project uses GitHub Actions for automated builds and releases:

#### CI/CD Pipeline
- **Continuous Integration**: Runs on every push and PR
- **Cross-platform Testing**: Tests on Linux, Windows, and macOS
- **Automated Releases**: Creates releases when tags are pushed
- **Nightly Builds**: Daily builds for testing latest changes

#### Release Process

1. **Create a tag**:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

2. **Automated Release**: GitHub Actions will:
   - Build for all platforms
   - Run tests and quality checks
   - Create distributable packages
   - Upload assets to GitHub Releases
   - Publish the release

3. **Development Builds**

For the latest development version:
- Check the [Actions](../../actions) tab for nightly builds
- Download artifacts from successful workflow runs
- Note: Development builds may be unstable

## 💻 Usage

### Medical Queries
The AI assistant is designed to help with:
- **Symptom Analysis**: Understanding patient presentations
- **Differential Diagnosis**: Exploring possible conditions
- **Treatment Options**: Discussing therapeutic approaches
- **Medical Education**: Learning about conditions and procedures
- **Clinical Decision Support**: Getting second opinions

### Example Interactions
```
User: "Patient presents with chest pain, shortness of breath, and diaphoresis. What should I consider?"

AI: "Based on these symptoms, you should consider several serious conditions in your differential diagnosis:

1. **Acute Coronary Syndrome** - The combination of chest pain, dyspnea, and diaphoresis is classic for MI
2. **Pulmonary Embolism** - Can present similarly, especially with sudden onset
3. **Pneumothorax** - Consider in young patients or those with underlying lung disease
..."
```

### Safety Features
- **Medical Disclaimers**: All responses include appropriate warnings
- **Educational Focus**: Emphasizes learning over direct patient care guidance
- **Context Awareness**: Maintains conversation history for follow-up questions

## 🛠️ Development

### Project Structure
```
offlinedoctor3/
├── src/                    # React frontend
│   ├── App.tsx            # Main application component
│   └── ...
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── ai_engine.rs   # AI inference management
│   │   ├── database.rs    # SQLite operations
│   │   ├── model_manager.rs # Model download/management
│   │   └── lib.rs         # Main application logic
│   └── Cargo.toml
├── package.json
└── README.md
```

### Building for Production

```bash
# Build the application
npm run tauri build

# The built application will be in src-tauri/target/release/bundle/
```

### Adding New Models

Edit `src-tauri/src/model_manager.rs` to add new model configurations:

```rust
ModelInfo {
    name: "New Medical Model".to_string(),
    size: 5_000_000_000,
    description: "Description of the model".to_string(),
    download_url: "https://huggingface.co/path/to/model.gguf".to_string(),
    filename: "model-name.gguf".to_string(),
    is_downloaded: self.is_model_downloaded("model-name.gguf"),
},
```

## 🧠 Model Information

### Supported Models
- **GGUF Format**: Quantized models for efficient inference
- **Size Range**: 2GB to 8GB (depending on quality needs)
- **Medical Training**: Preference for biomedical or medical-trained models
- **Quantization**: Q4, Q5, Q8 variants supported

### Performance Guidelines
- **2-4GB models**: 1-3 tokens/second on average hardware
- **RAM Requirements**: Model size + 2GB overhead
- **CPU vs GPU**: Currently CPU-only for maximum compatibility

## 🔧 Configuration

### AI Engine Settings
Located in `src-tauri/src/ai_engine.rs`:
- **Context Size**: Default 4096 tokens
- **Temperature**: 0.7 for balanced creativity/accuracy
- **Thread Count**: Adjustable based on CPU cores

### Database Configuration
SQLite database automatically created in application data directory:
- **Conversations**: Stored with metadata
- **Messages**: Full conversation history
- **Model Settings**: User preferences

## 🐛 Troubleshooting

### Common Issues

1. **"llama.cpp binary not found"**
   - Ensure llama-server is installed and in PATH
   - Check installation with: `which llama-server`

2. **Model download fails**
   - Check internet connection
   - Verify sufficient disk space
   - Try downloading manually from HuggingFace

3. **AI responses are slow**
   - Use smaller models (3B instead of 8B)
   - Increase thread count in AI settings
   - Ensure sufficient RAM

4. **Application won't start**
   - Check Rust/Node.js versions
   - Verify all dependencies installed
   - Check console for error messages

## 🤝 Contributing

We welcome contributions from the medical and developer communities! This project is especially valuable with input from healthcare professionals.

### How to Contribute

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**: Follow our coding standards and add tests
4. **Test thoroughly**: Ensure medical accuracy and safety
5. **Submit a pull request**: Use our PR template

### Types of Contributions

- **Medical Expertise**: Help improve medical prompts and accuracy
- **New Models**: Add support for additional medical AI models
- **Bug Fixes**: Report and fix issues
- **Documentation**: Improve guides and medical disclaimers
- **Performance**: Optimize inference speed and memory usage
- **UI/UX**: Enhance the user interface for healthcare workflows

### Development Guidelines

- **Medical Safety First**: All changes must maintain medical disclaimers
- **Code Quality**: Follow Rust and TypeScript best practices
- **Testing**: Add tests for new functionality
- **Documentation**: Update relevant documentation
- **Security**: Follow our security policy for medical applications

### Reporting Issues

Please use our issue templates:
- [Bug Report](.github/ISSUE_TEMPLATE/bug_report.md)
- [Feature Request](.github/ISSUE_TEMPLATE/feature_request.md)
- [Medical Model Request](.github/ISSUE_TEMPLATE/model_request.md)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Medical Disclaimer

This software is provided for educational and research purposes only. It is not intended to:
- Replace professional medical advice, diagnosis, or treatment
- Be used for actual patient care decisions
- Provide definitive medical guidance

Always consult with qualified healthcare professionals for medical decisions. The developers assume no responsibility for any medical decisions made based on this software's output.

## 🔒 Security

Please review our [Security Policy](SECURITY.md) for information about:
- Reporting vulnerabilities
- Medical data protection
- Privacy considerations
- Security best practices

## 📋 Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes and releases.

---

**Made with ❤️ for healthcare professionals worldwide**
