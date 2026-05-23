# Cloud Cat
[![Hackatime](https://hackatime-badge.hackclub.com/U08D22QNUVD/CloudCat)](https://waka.hackclub.com)
[![Latest Release](https://img.shields.io/github/v/release/spacexplorer11/CloudCat?style=flat)](https://github.com/Spacexplorer11/CloudCat/releases/latest)
[![License](https://img.shields.io/github/license/spacexplorer11/CloudCat?style=flat)](LICENSE)

## What is CloudCat?

CloudCat is a remix of the iconic Chrome dinosaur game, you know, the one you see when you're offline.

You play as a cat strolling along while devious **rainclouds** try to rain on your parade. Hold up your umbrella to keep your fur dry!

## Why did I make it?

I made this game to have some fun, and to learn Rust! It is my first ever Rust project, so don't expect it to be perfect. It's made using Macroquad, a game library for Rust, and is heavily inspired by the original Chrome dinosaur game.

## How to play

[**Play in your browser →**](https://cloudcat.online) or grab the [latest release](https://github.com/Spacexplorer11/CloudCat/releases/latest) for your OS.

- **Space**, **click**, or **tap** to raise the umbrella
- The umbrella lasts **3 seconds** per use
- Survive as long as possible without getting rained on
- Your highscore is saved automatically - **but only once the game over screen appears, not if you quit mid-game**

> [!TIP]
> Got feedback or ideas? Fill in the [quick survey](https://tally.so/r/2EXNLe) or [open an issue](https://github.com/Spacexplorer11/CloudCat/issues/new?template=feature_request.yml)!

## Memory usage

Tested on a MacBook Air M1 (2020), v1.3.1 macOS ARM64 build, during active gameplay: **61.8 MB**.

<img width="1036" height="121" alt="Memory usage screenshot" src="https://github.com/user-attachments/assets/77422f05-c771-457a-831a-6ed1dc9ac098" />

## Running locally

**Make sure you're in the project root before running any of these commands.**

### Native (Rust)

```bash
cargo run
```

### Web (WASM)

> [!NOTE]
> Requires Python 3 for the local HTTP server. Only tested on macOS and Linux - may not work on Windows.

```bash
rm -rf web
cargo build --target wasm32-unknown-unknown --release
mkdir web
cp target/wasm32-unknown-unknown/release/cloudcat.wasm web/
cp index.html gl.js quad-storage.js sapp_jsutils.js web/
cp -r assets favicons web/
cd web && python3 -m http.server 3000
```

Then open **http://localhost:3000** (not https! The local server is plain HTTP).

## Tech

Built with [Macroquad](https://macroquad.rs/) in Rust, compiled to WebAssembly for the browser version.

## AI transparency

| Area                               | AI usage                                                 |
|------------------------------------|----------------------------------------------------------|
| Gameplay & screens                 | Minimal (~5–10%)                                         |
| Game assets (cat, cloud, umbrella) | **None** - hand-drawn and hand-animated 🎨               |
| Web/WebGL deployment               | Heavy (~90%)                                             |
| Release notes                      | Full (100%) - auto-generated                             |
| GitHub Actions workflows           | Full (100%) - generated, hoping to learn YAML eventually |

Overall, the actual gameplay, concept, and art are almost entirely my own work which I'm very proud of!