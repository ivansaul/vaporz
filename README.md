<div align="center">

<h1 style="border-bottom: none">
    <b><a href="#">vaporz</a></b>
</h1>

Easily find and remove old and heavy build artifacts ✨

Clean up `node_modules`, `target`, `build`, `.venv`, and more — free space in seconds. 💥

</div>

<p align="center">
  <img src="https://img.shields.io/badge/Python-snow?logo=python&logoColor=3776AB" />
  <img src="https://img.shields.io/badge/Java-snow?logo=coffeescript&logoColor=FC4C02" />
  <img src="https://img.shields.io/badge/C++-snow?logo=c%2B%2B&logoColor=00599C" />
  <img src="https://img.shields.io/badge/C-snow?logo=c&logoColor=A8B9CC" />
  <img src="https://img.shields.io/badge/C%23-snow?logo=csharp&logoColor=512BD4" />
  <img src="https://img.shields.io/badge/JavaScript-snow?logo=javascript&logoColor=E9CE30" />
  <img src="https://img.shields.io/badge/Go-snow?logo=go&logoColor=00ADD8" />
  <img src="https://img.shields.io/badge/Swift-snow?logo=swift&logoColor=F05138" />
  <img src="https://img.shields.io/badge/Rust-snow?logo=rust&logoColor=000000" />
  <img src="https://img.shields.io/badge/Ruby-snow?logo=ruby&logoColor=CC342D" />
  <img src="https://img.shields.io/badge/Kotlin-snow?logo=kotlin&logoColor=7F52FF" />
  <img src="https://img.shields.io/badge/TypeScript-snow?logo=typescript&logoColor=3178C6" />
  <img src="https://img.shields.io/badge/Dart-snow?logo=dart&logoColor=0175C2" />
</p>

---

![demo](https://github.com/user-attachments/assets/7d294e58-ca1a-460c-8fc2-855b97848507)

## Features ✨

- **Clear space**: Remove build, dependency, and cache folders from `Rust`, `Python`, `Node`, `Flutter`, `Go`, `Java`, `.NET`, `Swift`, and more.
- **Last modified**: See when each project was last updated (`modified` column).
- **Fast**: Built in **Rust** for efficient and safe scanning, even on large directories.
- **Simple & intuitive**: Navigate with arrow keys in a clean `Ratatui` interface — press **Enter** to delete.
- **Lightweight**: Minimal dependencies, cross-platform, and easy to run anywhere.

## Installation

Prebuilt binaries are available for **Linux**, **macOS**, and **Windows** on the [Releases](https://github.com/ivansaul/vaporz/releases) page.

1. Download the latest release for your platform.
2. Extract the archive.
3. (Linux/macOS) Add executable permissions to the binary `chmod +x vaporz`.
4. Move the binary to a directory in your `$PATH`.

You can also run it directly without installing:

```console
./vaporz
```

## Usage

Navigate to the directory where your projects are located and run:

```console
cd /path/to/projects
vaporz
```

### Controls

- Navigate up/down (<kbd>↑</kbd> / <kbd>↓</kbd>)
- Remove selected (<kbd>Enter</kbd>)
- Quit (<kbd>Esc</kbd> or <kbd>q</kbd>)
- Sort by size (<kbd>s</kbd>)
- Sort by last modification time (<kbd>m</kbd>)
- Sort by path (<kbd>p</kbd>)


> [!IMPORTANT]
> `vaporz` acts like a `rm -rf` with a TUI interface. Use it with caution, always have a backup of your project before using it.

## Building

To build `vaporz` from source you need the [Rust](https://www.rust-lang.org/) toolchain (`rustc` + `cargo`). If it’s not available via your system package manager, install it using [rustup](https://rustup.rs/).

### Install via Cargo

To download the source code, build the vaporz binary, and install it in `$HOME/.cargo/bin`, run:

```console
cargo install --locked --git https://github.com/ivansaul/vaporz
```

This installs the binary into `$HOME/.cargo/bin`. Make sure that directory is in your `$PATH`.

### Manual Build

Alternatively, you can manually download the source code and build the vaporz binary with:

```console
git clone https://github.com/ivansaul/vaporz
cd vaporz
cargo build --release
```

The binary will be available at `target/release/vaporz`. Move it somewhere in your `$PATH` (e.g., `/usr/local/bin` or `$HOME/.local/bin`).

## Roadmap

- [ ] Fix bugs
- [ ] Code cleanup
- [ ] Performance optimizations
- [ ] Support more languages & frameworks

## Motivation

This project was originally built to learn Rust, so some parts may still be improved.
It started with the heavy `target` folders left behind in Rust projects, which can take up a lot of space, and the need to remove them easily. From there, it quickly extended to other ecosystems I use often, like Swift, Python, and Dart.

## Notes

This project is inspired on the [npkill](https://github.com/voidcosmos/npkill) project.

## Contributors

<a href="https://github.com/ivansaul/vaporz/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=ivansaul/vaporz" />
</a>

## Contribute

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](https://github.com/ivansaul/vaporz/blob/main/LICENSE).
