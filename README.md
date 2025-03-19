# Rust Audio App

## Overview
The Rust Audio App is a macOS application that allows users to record audio, organize recorded files, transcribe audio using Whisper models, and summarize the transcriptions using a local or API-based language model.

## Features
- **Audio Recording**: Record audio through the application.
- **File Organization**: Organize recorded audio files into folders and manage them efficiently.
- **Transcription**: Utilize Whisper models to transcribe recorded audio files into text.
- **Summarization**: Access an API or run a local language model to summarize the transcribed text.

## Project Structure
```
rust-audio-app
├── src
│   ├── main.rs          # Entry point of the application
│   ├── audio
│   │   └── recorder.rs  # Audio recording functionality
│   ├── organizer
│   │   └── mod.rs      # File organization functionality
│   ├── transcription
│   │   └── mod.rs      # Audio transcription functionality
│   ├── summarization
│   │   └── mod.rs      # Text summarization functionality
│   └── utils.rs         # Utility functions
├── Cargo.toml           # Cargo configuration file
└── README.md            # Project documentation
```

## Getting Started

### Prerequisites
- Install Rust: Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
- Ensure you have the necessary libraries for audio recording and Whisper model integration.

### Installation
1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-audio-app.git
   cd rust-audio-app
   ```

2. Build the project:
   ```
   cargo build
   ```

### Usage
1. Run the application:
   ```
   cargo run
   ```

2. Follow the on-screen instructions to record audio, organize files, transcribe audio, and summarize the transcriptions.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License
This project is licensed under the MIT License - see the LICENSE file for details.