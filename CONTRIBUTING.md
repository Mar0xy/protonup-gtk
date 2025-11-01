# Contributing to ProtonUp-GTK

Thank you for your interest in contributing to ProtonUp-GTK! This document provides guidelines and information for contributors.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/protonup-gtk.git`
3. Install dependencies (see README.md)
4. Create a new branch: `git checkout -b feature/your-feature-name`

## Development Setup

### Prerequisites

You'll need:
- Rust (latest stable)
- GTK4 development libraries
- libadwaita development libraries
- pkg-config

See the README.md for platform-specific installation instructions.

### Building

```bash
# Build with GUI support (requires GTK4/libadwaita)
cargo build --features gui

# Build backend only (no GUI dependencies)
cargo build --no-default-features

# Run tests
cargo test --no-default-features
```

### Running

```bash
# Run the application
cargo run --features gui

# Run backend tests
cargo run --example test_github_api --no-default-features
```

## Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy for linting: `cargo clippy`
- Write clear, self-documenting code
- Add comments for complex logic
- Write tests for new functionality

## Adding New Compatibility Tools

To add a new compatibility tool:

1. Add a new fetch method in `src/backend/tool_manager.rs`
2. Update the `fetch_available_tools()` method to call your new method
3. Update the UI in `src/window.rs` to display the new tool
4. Update the README.md to document the new tool
5. Add a test case in the test example

Example:
```rust
async fn fetch_your_tool_latest(&self) -> Result<CompatibilityTool> {
    let url = "https://api.github.com/repos/owner/repo/releases/latest";
    let release: GitHubRelease = self.client
        .get(url)
        .send()
        .await?
        .json()
        .await?;

    let asset = release.assets
        .iter()
        .find(|a| a.name.ends_with(".tar.gz"))
        .ok_or_else(|| anyhow::anyhow!("No asset found"))?;

    Ok(CompatibilityTool {
        name: "Your Tool".to_string(),
        description: "Description of your tool".to_string(),
        launcher: Launcher::Steam, // or Launcher::Lutris
        download_url: asset.browser_download_url.clone(),
        version: release.tag_name.clone(),
    })
}
```

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update the CHANGELOG.md (if it exists) with your changes
3. Ensure all tests pass
4. Run `cargo fmt` and `cargo clippy` before submitting
5. Write a clear PR description explaining your changes
6. Link any related issues

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

## Questions?

Feel free to open an issue for:
- Bug reports
- Feature requests
- Questions about the code
- Documentation improvements

## License

By contributing, you agree that your contributions will be licensed under the GPL-3.0-or-later license.
