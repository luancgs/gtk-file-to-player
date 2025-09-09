# GTK File to Player

A GTK4-based media player application written in Rust that provides a searchable interface for playing audio and video files from a SQLite database.

## Description

This application is a media library player that combines a GTK4 user interface with VLC media player backend. It reads media file information from a SQLite database and provides a search interface to quickly find and play songs or videos. The application is designed to work with pre-catalogued media collections stored in a database.

## Requirements

- **Rust**: Latest stable version (2024 edition)
- **GTK4**: Version 4.10 or higher
- **VLC Media Player**: Must be installed and available in PATH
- **SQLite database**: With a `song` table containing `number`, `title`, and `file` columns

## Installation

### Prerequisites

Install the required system dependencies:

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install libgtk-4-dev vlc build-essential

# Fedora
sudo dnf install gtk4-devel vlc gcc

# Arch Linux
sudo pacman -S gtk4 vlc base-devel
```

### Build from source

```bash
# Clone the repository
git clone https://github.com/luancgs/gtk-file-to-player.git
cd gtk-file-to-player

# Build the project
cargo build --release

# The binary will be available at target/release/gtk-file-to-player
```

## Configuration

1. Copy the sample configuration file:
   ```bash
   cp sample.config.toml config.toml
   ```

2. Edit `config.toml` with your settings:
   ```toml
   database_path = "/path/to/your/media.db"
   song_directory = "/path/to/your/media/files"
   window_title = "My Media Player"
   search_placeholder = "Search songs and videos..."
   ```

### Database Schema

Your SQLite database should contain a `song` table with the following structure:

```sql
CREATE TABLE song (
    number INTEGER,
    title TEXT,
    file TEXT
);
```

Where:
- `number`: Song/video number for ordering and search
- `title`: Display title of the media file
- `file`: Filename relative to the `song_directory` configured path

## Usage

1. **Start the application**:
   ```bash
   ./target/release/gtk-file-to-player
   ```

2. **Search for media**:
   - Type in the search box to find songs by number or title
   - Results appear instantly as you type
   - Search is limited to 10 results for performance

3. **Play media**:
   - Click any song button to start playback in VLC
   - VLC will open with the selected media file
   - Desktop notifications show the currently playing track

## Project Structure

```
src/
├── main.rs          # Application entry point and setup
├── app.rs           # Main window and UI coordination
├── config.rs        # Configuration loading and validation
├── database.rs      # SQLite database operations
├── player.rs        # VLC media player integration
└── ui.rs            # UI components and error dialogs
```

## Dependencies

- **gtk4**: GTK4 bindings for Rust
- **sqlite**: SQLite database interface
- **serde**: Serialization framework for configuration
- **toml**: TOML configuration parser

## Error Handling

The application provides comprehensive error handling for:
- Missing or invalid configuration files
- Database connection issues
- VLC availability checks
- File system access problems
- Media playback errors

Errors are displayed through GTK dialogs and desktop notifications.

## Development

### Running tests

```bash
cargo test
```

### Debug build

```bash
cargo build
./target/debug/gtk-file-to-player
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Troubleshooting

**VLC not found**: Ensure VLC is installed and available in your system PATH.

**Database errors**: Verify your database file exists and contains the required `song` table.

**Configuration errors**: Check that your `config.toml` file is valid and all paths exist.

**GTK4 issues**: Ensure you have GTK4 development libraries installed for your distribution.
