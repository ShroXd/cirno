# Cirno

<div align="center">

![Cirno Logo](path/to/your/logo.png)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![GitHub Stars](https://img.shields.io/github/stars/yourusername/cirno?style=social)](https://github.com/yourusername/cirno)
[![Docker Pulls](https://img.shields.io/docker/pulls/yourusername/cirno)](https://hub.docker.com/r/yourusername/cirno)

_Your Personal Media Management Platform_

[Features](#features) • [Installation](#installation) • [Quick Start](#quick-start) • [Documentation](#documentation) • [Contributing](#contributing) • [Support](#support)

</div>

## 🎯 Overview

Cirno is a modern, self-hosted media management platform that helps you organize and stream your video content. Whether you're managing TV shows, movies, or personal videos, Cirno provides a sleek, intuitive interface for your media library.

## ✨ Features

- **Smart Library Management**

  - Automatic metadata fetching and organization
  - Custom collections and playlists
  - Advanced search and filtering capabilities

- **Powerful Streaming**

  - Direct play and transcoding support
  - Adaptive streaming quality
  - Resume playback across devices

- **Customization**

  - Personalizable user interface
  - Custom metadata fields
  - Flexible organization systems
  - Multi-language support

## 🚀 Quick Start

<!-- ### Using Docker (Recommended)

```bash
docker run -d \
  --name cirno \
  -p 8096:8096 \
  -v /path/to/media:/media \
  -v /path/to/config:/config \
  yourusername/cirno:latest
``` -->

### Manual Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/ShroXd/cirno.git
   cd cirno
   ```

2. **Install taskfile**

   ```bash
   curl -fsSL https://taskfile.dev/install.sh | sh
   ```

3. **Install dependencies**

   ```bash
   task install
   ```

4. **Start the server**

   ```bash
   task start
   ```

Visit `http://localhost:5173/` to access your Cirno instance.

## 📖 Documentation

For detailed information about setup, configuration, and usage, visit our [Documentation](https://docs.cirno.dev).

### System Requirements

- Docker
- Modern web browser
- Sufficient storage space for your media
- 2GB RAM minimum (4GB recommended)

## 🛠️ Development

For development tasks, you can find all available commands in the `Taskfile.yml`. Here are some common commands:

```bash
# Install development dependencies
task install

# Run in development mode
task dev

# Run tests
task test

# Build for production
task build
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 💖 Support

- Star this repository
- Report issues
- Submit pull requests
- Share with others

## 🙏 Acknowledgments

- [Cursor](https://cursor.sh/) for AI-powered code editing
- [Claude](https://www.anthropic.com/claude) for help with development
- [ChatGPT](https://chat.openai.com/) for assistance with code generation
- [Gemini](https://gemini.google.com/) for code assistance and documentation
- [NotebookLM](https://notebooklm.google/) for research and documentation
- [GStreamer](https://gstreamer.freedesktop.org/) for video processing
- [SQLite](https://www.sqlite.org/) for data storage

---

<div align="center">
Made with ❤️ by the Cirno Team
</div>
