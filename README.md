# Dioxus Bevy Example

A Dioxus web application that embeds a 3D Bevy scene into a custom canvas element.

This embeds the Bevy "generate_custom_mesh" example into a Dioxus web app. It also demonstrates
interactions between the Dioxus UI and the Bevy view with a HTML button that sends an event into the
Bevy app.

## Development

### Prerequisites

- Rust (latest stable)
- `dx` CLI tool for Dioxus development (0.7)
- WASM target: `rustup target add wasm32-unknown-unknown`

### Running the App

1. Install the Dioxus CLI:
```bash
cargo install dioxus-cli
```

2. Serve the application:
```bash
dx serve
```

3. Hit "o" to open a browser.
