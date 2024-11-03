# Discord Attendance Bot

A Discord bot built with Rust that helps track and manage attendance in Discord servers. This bot provides simple attendance tracking commands while ensuring secure admin access and offering extensibility through API integration.

## Features

### Core Features
- **Attendance Tracking**
  - Track user attendance through Discord commands
  - View attendance history and statistics
  - Export attendance data in various formats

- **Secure Admin Authentication**
  - Admin-only commands and features
  - Role-based access control
  - Server-specific configuration

### Planned Features
- **REST API Integration**
  - HTTP endpoints for bot control
  - Webhook support for events
  - API authentication and rate limiting
  - Integration capabilities with external applications

- **Terminal Application**
  - Command-line interface for bot management
  - Real-time monitoring and logging
  - Direct bot control without Discord interface

## Getting Started

### Prerequisites
- Rust (latest stable version)
- Discord Bot Token
- PostgreSQL (for storing attendance data)

### Installation
1. Clone the repository
```bash
git clone https://github.com/yourusername/discord-attendance-bot
cd discord-attendance-bot
```

2. Configure environment variables
```bash
cp .env.example .env
# Edit .env with your Discord token and database credentials
```

3. Build and run
```bash
cargo build --release
cargo run
```

## Usage

### Basic Commands
```
/attendance check - Mark your attendance
/attendance status - Check your attendance status
/attendance history - View your attendance history
```

### Admin Commands
```
/admin login - Authenticate as admin
/admin config - Configure bot settings
/admin report - Generate attendance reports
```

## Configuration

The bot can be configured through either:
- Environment variables
- Configuration file (`config.toml`)
- Discord admin commands

## Security

- Admin authentication is required for sensitive operations
- Bot tokens and credentials must be kept secure
- Rate limiting is implemented to prevent abuse
- Role-based access control for different command levels

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

### Development Setup
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

If you have any questions or need help with setup, please:
- Open an issue in the GitHub repository
- Join our [Discord support server](#)
- Check the [documentation](#)

## Roadmap

- [x] Basic attendance tracking
- [x] Admin authentication
- [ ] REST API implementation
- [ ] Terminal application
- [ ] Advanced reporting features
- [ ] Multi-server support

## Acknowledgments

- Discord API documentation
- Rust Discord community
- Contributors and supporters
