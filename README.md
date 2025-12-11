# ghviz

A command-line tool to visualize GitHub commit history in a beautifully formatted, colored output.

## Features

- Fetch commit history from any public GitHub repository
- Colorized output with formatted commit information
- ASCII art header
- Configurable commit limit
- Displays SHA, author, message, date, and commit URL

## Installation

```bash
git clone https://github.com/adityajha2005/ghviz
cd ghviz
cargo build --release
```

## Usage

```bash
ghviz <owner/repo> [OPTIONS]
```

### Arguments

- `repo` - GitHub repository in the format `owner/repo` (e.g., `rust-lang/rust`)

### Options

- `-l, --limit <limit>` - Maximum number of commits to display (default: 10)

### Examples

```bash
# Display last 10 commits from rust-lang/rust
ghviz rust-lang/rust

# Display last 25 commits from microsoft/vscode
ghviz microsoft/vscode --limit 25

# Short form
ghviz axumrs/axum -l 5
```

## Output Format

The tool displays commits with:
- **SHA**: Short commit hash (first 7 characters)
- **Author**: Commit author name
- **Message**: Commit message
- **Date**: Commit date in YYYY-MM-DD format
- **URL**: Link to the commit on GitHub

## Dependencies

- `reqwest` - HTTP client for GitHub API requests
- `tokio` - Async runtime
- `serde` - Serialization/deserialization
- `clap` - Command-line argument parsing
- `colored` - Terminal colors
- `chrono` - Date/time handling
- `figlet-rs` - ASCII art generation

