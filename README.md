# Zenith Browser

Zenith is a high-performance, lightweight, and minimalistic web browser built with Rust. It uses the `wry` and `tao` crates to deliver a native browsing experience with a modern, glassmorphic UI shell.

## Features

- **Blazing Fast**: Leverages the system's native WebView engine (WKWebView on macOS).
- **Minimalistic UI**: A clean, distraction-free interface with a glassmorphism toolbar.
- **Privacy Oriented**: No tracking, no bloat, just the web.
- **Single Binary**: The entire browser, including UI assets, is compiled into a single executable.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- macOS (Core support) or Linux/Windows (requires minor build adjustments for WebKitGTK/WebView2)

## Installation

```bash
git clone https://github.com/your-username/zenith-browser.git
cd zenith-browser
cargo run --release
```

## Usage

- **Navigation**: Enter a URL or search query in the address bar and press **Enter**.
- **Controls**: Use the back, forward, and reload buttons on the left.
- **Search**: If the input is not a valid URL, it defaults to a Google search.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
