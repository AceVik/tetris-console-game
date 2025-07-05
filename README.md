# Console Tetris Game

[![Build Status](https://github.com/AceVik/tetris-console-game/actions/workflows/release.yml/badge.svg)](https://github.com/AceVik/tetris-console-game/actions)
[![Latest Release](https://img.shields.io/github/v/release/AceVik/tetris-console-game)](https://github.com/AceVik/tetris-console-game/releases/latest)

A simple Tetris game that runs in your console/terminal. Built with Rust for learning purposes.

![Demo](.github/assets/tetris.avif)

## Game Controls

| Key | Action |
|-----|--------|
| A | Move tetromino left |
| D | Move tetromino right |
| S | Move tetromino down |
| W | Drop tetromino to the bottom |
| R | Rotate tetromino |
| Space | Pause/Unpause game |
| Backspace | Reset game |
| Ctrl+C | Exit game |

## Future Plans 🚀

Here are some of the features and improvements planned for future versions:

- **Wall Kicks**: Implement "wall kicks" to allow rotating pieces near the edge of the board by shifting them away from the wall if there is space.
- **Hold Queue**: Add a "hold" feature allowing the player to swap the current tetromino with a stored one using the Q and E (if two) keys, with a big score penalty for each swap, to make getting high score harder.
- **Seedable Games**: Introduce a seeding mechanism to allow for reproducible tetromino sequences, making specific challenges or races possible.
- **Replay System**: Track all game events to create a replay system, allowing players to watch and share their games.
- **Online Leaderboard**: Develop a global leaderboard to display high scores, complete with links to replays.

## Prerequisites

### Installing Rust

To run or build this game, you need to have Rust installed on your system. If you don't have Rust installed, follow these steps:

1. Visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Follow the instructions for your operating system:
    - **Windows**: Download and run the rustup-init.exe file
    - **macOS/Linux**: Run the following command in your terminal:
      ```bash
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```
3. Follow the on-screen instructions to complete the installation
4. Verify the installation by running:
   ```bash
   rustc --version
   cargo --version
   ```

## Running the Game

### Clone the Repository

```bash
# Using HTTPS
git clone https://github.com/AceVik/tetris-console-game.git

# Or using SSH
git clone git@github.com:AceVik/tetris-console-game.git
cd tetris-console-game
```

### Run in Debug Mode

```bash
cargo run
```

### Run in Release Mode (better performance)

```bash
cargo run --release
```

## Building the Game

### Debug Build

```bash
cargo build
```

The executable will be located at `target/debug/tetris-console-game` (or `target\debug\tetris-console-game.exe` on Windows).

### Release Build (optimized)

```bash
cargo build --release
```

The executable will be located at `target/release/tetris-console-game` (or `target\release\tetris-console-game.exe` on Windows).

## Running Tests

To run all tests:

```bash
cargo test
```

To run tests with verbose output:

```bash
cargo test -- --nocapture
```

## Dependencies

- **chrono**: Date and time functionality
- **crossterm**: Terminal manipulation
- **rand**: Random number generation
- **rodio**: Audio playback

## Note

You may need to install additional system dependencies for the audio functionality (rodio crate) depending on your operating system.
