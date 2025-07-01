# Console Tetris Game

[![Build Status](https://github.com/AceVik/tetris-console-game/actions/workflows/release.yml/badge.svg)](https://github.com/AceVik/tetris-console-game/actions)
[![Latest Release](https://img.shields.io/github/v/release/AceVik/tetris-console-game)](https://github.com/AceVik/tetris-console-game/releases/latest)

A simple Tetris game that runs in your console/terminal. Built with Rust for learning purposes.

## Prerequisites

### Installing Rust

To run or build this game, you need to have Rust installed on your system. If you don't have Rust installed, follow these steps:

1. Visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Follow the instructions for your operating system:
    - **Windows**: Download and run the rustup-init.exe file
    - **macOS/Linux**: Run the following command in your terminal:
      ```
      curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
      ```
3. Follow the on-screen instructions to complete the installation
4. Verify the installation by running:
   rustc --versioncargo --version
## Running the Game

### Clone the Repository

git clone https://github.com/AceVik/tetris-console-game.git // Orgit clone git@github.com:AceVik/tetris-console-game.gitcd tetris-console-game
### Run in Debug Mode

cargo run
### Run in Release Mode (better performance)

cargo run --release
## Building the Game

### Debug Build

cargo build
The executable will be located at `target/debug/tetris-console-game` (or `target\debug\tetris-console-game.exe` on Windows).

### Release Build (optimized)

cargo build --release
The executable will be located at `target/release/tetris-console-game` (or `target\release\tetris-console-game.exe` on Windows).

## Running Tests

To run all tests:

cargo test
To run tests with verbose output:

cargo test -- --nocapture
## Game Controls

- `a` - Move tetromino left
- `d` - Move tetromino right
- `s` - Move tetromino down
- `w` - Drop tetromino to the bottom
- `r` - Rotate tetromino
- `Space` - Pause/Unpause game
- `Backspace` - Reset game
- `Ctrl+C` - Exit game

## Dependencies

- chrono: Date and time functionality
- crossterm: Terminal manipulation
- rand: Random number generation
- rodio: Audio playback

## Note

You may need to install additional system dependencies for the audio functionality (rodio crate) depending on your operating system.
