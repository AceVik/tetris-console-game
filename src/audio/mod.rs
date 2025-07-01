use crate::game::audio;
use rand::Rng;
use rodio::{source::Source, Decoder, OutputStreamHandle, Sink};
use std::io::Cursor;
use std::sync::{Arc, Mutex, PoisonError};

const BACKGROUND_MUSIC: &[u8] = include_bytes!("../../assets/audio/background_music.ogg");
const GAME_OVER_SOUND: &[u8] = include_bytes!("../../assets/audio/game_over.mp3");
const FUNNY_GAME_OVER_SOUND: &[u8] = include_bytes!("../../assets/audio/funny_game_over.mp3");
const PAUSE_SOUND: &[u8] = include_bytes!("../../assets/audio/pause.mp3");
const LINE_CLEAR_SOUND: &[u8] = include_bytes!("../../assets/audio/line_clear.mp3");
const TETRIS_LINE_CLEAR_SOUND: &[u8] = include_bytes!("../../assets/audio/tetris_line_clear.mp3");
const MOVE_SOUND: &[u8] = include_bytes!("../../assets/audio/move.mp3");
const ROTATE_SOUND: &[u8] = MOVE_SOUND;
const DROP_SOUND: &[u8] = include_bytes!("../../assets/audio/drop.mp3");
const LEVEL_UP_SOUND: &[u8] = include_bytes!("../../assets/audio/level_up.mp3");
const NO_MOVE_SOUND: &[u8] = include_bytes!("../../assets/audio/no_move.mp3");
const NO_ROTATE_SOUND: &[u8] = NO_MOVE_SOUND;

pub struct AudioPlayer {
    stream_handle: Arc<OutputStreamHandle>,
    background_music_sink: Arc<Mutex<Option<Sink>>>,
    is_effects_muted: Arc<Mutex<bool>>,
}

impl AudioPlayer {
    pub fn new(stream_handle: Arc<OutputStreamHandle>) -> Self {
        let background_music_sink = Sink::try_new(&stream_handle)
            .map(|sink| Arc::new(Mutex::new(Some(sink))))
            .unwrap_or_else(|_| Arc::new(Mutex::new(None)));

        Self {
            stream_handle,
            background_music_sink,
            is_effects_muted: Arc::new(Mutex::new(false)),
        }
    }

    fn play_effect_sound(&self, sound_data: &'static [u8]) {
        if *self
            .is_effects_muted
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
        {
            return;
        }

        let audio_cursor = Cursor::new(sound_data);
        if let Ok(source) = Decoder::new(audio_cursor) {
            if let Ok(sink) = Sink::try_new(&self.stream_handle) {
                sink.append(source);
                sink.detach();
            }
        }
    }
}

impl audio::AudioPlayer for AudioPlayer {
    fn play_background_music(&self) {
        let Some(mut optional_sink_guard) = self.background_music_sink.lock().ok() else {
            return;
        };
        let Some(sink) = optional_sink_guard.as_mut() else {
            return;
        };

        if sink.empty() {
            let audio_cursor = Cursor::new(BACKGROUND_MUSIC);
            let Ok(source) = Decoder::new(audio_cursor) else {
                return;
            };
            sink.append(source.repeat_infinite());
        }
        sink.play();
    }

    fn play_game_over_sound(&self) {
        let mut rng = rand::rng();
        let use_funny_sound = rng.random_bool(0.01);

        let sound_to_play = if use_funny_sound {
            FUNNY_GAME_OVER_SOUND
        } else {
            GAME_OVER_SOUND
        };
        self.play_effect_sound(sound_to_play);
    }

    fn play_pause_sound(&self) {
        self.play_effect_sound(PAUSE_SOUND);
    }

    fn play_line_clear_sound(&self) {
        self.play_effect_sound(LINE_CLEAR_SOUND);
    }

    fn play_tetris_line_clear_sound(&self) {
        self.play_effect_sound(TETRIS_LINE_CLEAR_SOUND);
    }

    fn play_move_sound(&self) {
        self.play_effect_sound(MOVE_SOUND);
    }

    fn play_no_move_sound(&self) {
        self.play_effect_sound(NO_MOVE_SOUND);
    }

    fn play_rotate_sound(&self) {
        self.play_effect_sound(ROTATE_SOUND);
    }

    fn play_no_rotate_sound(&self) {
        self.play_effect_sound(NO_ROTATE_SOUND);
    }

    fn play_drop_sound(&self) {
        self.play_effect_sound(DROP_SOUND);
    }

    fn play_level_up_sound(&self) {
        self.play_effect_sound(LEVEL_UP_SOUND);
    }

    fn mute_background_music(&mut self) {
        if let Ok(mut guard) = self.background_music_sink.lock() {
            if let Some(sink) = guard.as_mut() {
                if sink.is_paused() {
                    sink.play();
                } else {
                    sink.pause();
                }
            }
        }
    }

    fn mute_effects(&mut self) {
        if let Ok(mut is_muted) = self.is_effects_muted.lock() {
            *is_muted = !*is_muted;
        }
    }
}
