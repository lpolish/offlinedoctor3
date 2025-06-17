<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

# Offline Doctor AI - Copilot Instructions

This is a Tauri application that provides an offline medical AI assistant for healthcare professionals, particularly resident doctors in remote locations.

## Project Structure
- **Frontend**: React + TypeScript with Tailwind CSS
- **Backend**: Rust-based Tauri application with offline AI capabilities
- **AI Engine**: Integration with llama.cpp for running language models locally
- **Database**: SQLite for conversation history
- **Models**: Support for downloading and running GGUF format models

## Key Features
- Completely offline AI inference using llama.cpp
- Medical-focused AI responses for healthcare professionals
- Conversation history and management
- Model download and management system
- Real-time chat interface

## Development Guidelines
- All AI responses must be genuine inference, never mocked or hardcoded
- Focus on medical accuracy and educational value
- Ensure offline functionality is maintained
- Prioritize performance for resource-constrained environments
- Follow medical disclaimer practices for AI-generated content

## Dependencies
- Tauri v2 with React frontend
- llama.cpp for AI inference
- SQLite for data persistence
- Tailwind CSS for styling
- Medical-focused language models (Llama, OpenBioLLM variants)

When working with this codebase, prioritize:
1. Offline functionality
2. Medical accuracy and safety
3. Performance optimization
4. User experience for healthcare workflows
