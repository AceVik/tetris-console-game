use crate::game::Game;
use crate::renderer::writers::size_screen;
use crate::renderer::Renderer;
use crossterm::event::{read, DisableFocusChange, EnableFocusChange, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{ClearType, SetSize};
use crossterm::{cursor, execute, terminal};
use rodio::OutputStream;
use std::io::{self, stdout};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod audio;
mod game;
mod renderer;

fn main() -> io::Result<()> {
    let screen_size = terminal::size()?;
    let raw_mode_enabled = terminal::is_raw_mode_enabled()?;
    if !raw_mode_enabled {
        terminal::enable_raw_mode()?;
    }
    execute!(stdout(), EnableFocusChange)?;

    run_game();

    if !raw_mode_enabled {
        terminal::disable_raw_mode()?;
    }
    execute!(
        stdout(),
        DisableFocusChange,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        SetSize(screen_size.0, screen_size.1)
    )?;

    Ok(())
}

fn run_game() {
    let renderer = Box::new(Renderer::new());

    let (_output_stream, stream_handle) = match OutputStream::try_default() {
        Ok(tuple) => tuple,
        Err(_) => {
            // If audio initialization fails, create dummy stream/handle to avoid panics
            // This is a workaround if OutputStream::try_default() fails
            // In a real scenario, you might want to handle this more gracefully,
            // e.g., by returning a Result from run_game or using a NoOpAudioPlayer.
            // For now, we'll just panic if it's truly unrecoverable, as try_default should work.
            panic!("Failed to initialize audio output stream. Is an audio device available?");
        }
    };
    let arc_stream_handle = Arc::new(stream_handle);

    let audio_player = Box::new(audio::AudioPlayer::new(arc_stream_handle.clone()));
    let game_mx = Arc::new(Mutex::new(Game::new(renderer, audio_player)));
    let game_mx_clone = Arc::clone(&game_mx);

    let game_thread = thread::spawn(move || {
        loop {
            {
                let mut game = game_mx_clone.lock().unwrap();
                if game.should_exit() {
                    break;
                }

                game.tick();
            }

            thread::sleep(Duration::from_millis(32));
        }
    });

    loop {
        let e = read().expect("Error reading inputs");
        let mut game = game_mx.lock().unwrap();

        match e {
            Event::FocusLost => {
                game.set_paused(true);
            }
            Event::Key(event) => {
                if event.is_release() {
                    continue; // Skip key release events to unwanted prevent double actions
                }
                
                if event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL {
                    game.exit();
                    break;
                } else if event.code == KeyCode::Char(' ') {
                    let paused = game.is_paused();
                    game.set_paused(!paused);
                } else if event.code == KeyCode::Backspace {
                    game.reset();
                } else if !game.is_paused() && !game.is_game_over() {
                    match event.code {
                        KeyCode::Char('r') => game.rotate_current_object(),
                        KeyCode::Char('a') => game.move_current_object_left(),
                        KeyCode::Char('d') => game.move_current_object_right(),
                        KeyCode::Char('s') => game.move_current_object_down(),
                        KeyCode::Char('w') => game.drop_current_object_down(),
                        _ => continue,
                    }
                }
            }
            Event::Resize(_, _) => size_screen(&mut stdout()).expect("resize failed"),
            _ => continue,
        }
    }

    game_thread.join().unwrap();
}
