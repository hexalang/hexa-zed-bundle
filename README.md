# hexa-zed-bundle

[Hexa](https://github.com/hexalang) support for **Zed** editor

<p align="center">
  <a href="https://github.com/hexalang/Hexa">
  <img width="256" src="https://raw.githubusercontent.com/hexalang/hexa-zed-bundle/kawaii/zed.webp">
  </a>
</p>

## Supported features

- Syntax highlighting with tree-sitter
- Language server connection

## Installation

- Install [rustup](https://www.rust-lang.org/tools/install)
- `git clone https://github.com/hexalang/hexa-zed-bundle.git`
- Open Extensions with `Ctrl + Shift + X`
- Click "Install Dev Extension" and select repo folder
- Wait for about 30 seconds for the extension to compile
- Extension should appear in the list of installed extensions

## Build manually

- Run `cargo build --target=wasm32-wasip1` inside the repo folder
