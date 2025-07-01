use crate::game::audio::AudioPlayer;
use crate::game::field::Field;
use crate::game::geometry::Direction;
use crate::game::object::Object;
use crate::game::rendering::Renderer;
use crate::game::tetromino::{Tetromino, TetrominoBag};
use chrono::Utc;
use std::ops::Div;

pub mod audio;
pub mod consts;
pub mod field;
pub mod geometry;
pub mod object;
pub mod rendering;
pub mod tetromino;

#[inline(always)]
pub fn calc_level_speed(level: u8) -> i64 {
    if level == 0 {
        panic!("Level must be greater than 0 for speed calculation.");
    }

    const INITIAL_INTERVAL_MS: f32 = 1000.0;
    const TARGET_INTERVAL_MS: f32 = 250.0;
    const LEVEL_AT_TARGET_INTERVAL: f32 = 50.0;

    let base_ratio = TARGET_INTERVAL_MS / INITIAL_INTERVAL_MS;
    let exponent_for_factor = 1.0 / (LEVEL_AT_TARGET_INTERVAL - 1.0);

    let scale_factor = base_ratio.powf(exponent_for_factor);

    let current_exponent = level as f32 - 1.0;

    let mut interval_ms_f32 = INITIAL_INTERVAL_MS * scale_factor.powf(current_exponent);

    interval_ms_f32 = interval_ms_f32
        .max(TARGET_INTERVAL_MS)
        .min(INITIAL_INTERVAL_MS);

    interval_ms_f32.round() as i64
}

pub struct Game {
    pub field: Field,
    tetromino_bag: TetrominoBag,
    pub current_object: Object,
    pub next_object: (Tetromino, Direction),
    pub level: u8,
    pub score: u64,
    paused: bool,
    pub game_over: bool,
    last_tick: i64,
    renderer: Box<dyn Renderer>,
    audio_player: Box<dyn AudioPlayer>,
    should_exit: bool,
}

impl Game {
    pub fn new(renderer: Box<dyn Renderer>, audio_player: Box<dyn AudioPlayer>) -> Self {
        let field = Field::new();
        let mut tetromino_bag = TetrominoBag::new();
        let tetromino = tetromino_bag.get_next_tetromino_and_update_weights();

        let direction = Direction::random();
        let pos = field
            .get_start_pos(&tetromino, &direction)
            .expect("Failed to get initial start position for tetromino!");

        let game = Game {
            field,
            current_object: Object::new(tetromino, direction, pos),
            next_object: (
                tetromino_bag.get_next_tetromino_and_update_weights(),
                Direction::random(),
            ),
            tetromino_bag,
            level: 1,
            score: 0,
            paused: true,
            game_over: false,
            should_exit: false,
            renderer,
            audio_player,
            last_tick: 0,
        };
        game.renderer.prerender();
        game.renderer.render(&game);
        game
    }

    pub fn reset(&mut self) {
        self.field.reset();

        self.tetromino_bag.reset_weights();
        let tetromino = self.tetromino_bag.get_next_tetromino_and_update_weights();

        let direction = Direction::random();
        let pos = self
            .field
            .get_start_pos(&tetromino, &direction)
            .expect("Failed to get initial start position for tetromino!");

        self.current_object = Object::new(tetromino, direction, pos);
        self.next_object = (
            self.tetromino_bag.get_next_tetromino_and_update_weights(),
            Direction::random(),
        );
        self.level = 1;
        self.score = 0;
        self.paused = true;
        self.game_over = false;
        self.last_tick = 0;

        self.renderer.prerender();
        self.renderer.render(self);
    }

    fn add_score(&mut self, points: u64) {
        self.score += points;

        // Increase the level based on the score
        if self.score >= self.level as u64 * 150 {
            self.level += 1;
            self.audio_player.play_level_up_sound();
        }
    }

    pub fn exit(&mut self) {
        self.should_exit = true;
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        if paused != self.paused {
            self.paused = paused;

            self.renderer.prerender();
            self.renderer.render(self);

            self.audio_player.play_pause_sound();
            if !self.paused {
                self.audio_player.play_background_music();
            } else {
                self.audio_player.mute_background_music();
            }
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn next(&mut self) {
        let (tetromino, direction) = self.next_object;
        match self.field.get_start_pos(&tetromino, &direction) {
            Some(pos) => {
                self.current_object = Object::new(tetromino, direction, pos);
                self.next_object = (
                    self.tetromino_bag.get_next_tetromino_and_update_weights(),
                    Direction::random(),
                );
            }

            None => {
                // If we cannot get a start position for the next tetromino, it means the game is over
                self.game_over = true;
                self.audio_player.mute_background_music();
                self.audio_player.play_game_over_sound();
            }
        }

        self.renderer.render(self);
    }

    pub fn rotate_current_object(&mut self) {
        let new_direction = self.current_object.get_direction().copy_rotate();
        let new_qube = self
            .current_object
            .get_type()
            .get_cube_by_direction(&new_direction);

        let new_qube_width = self.current_object.tetromino.dimensions(&new_direction).0;

        if self.field.can_hold(
            (&new_qube, new_qube_width),
            &self.current_object.get_position(),
        ) {
            self.current_object.rotate();
            self.audio_player.play_rotate_sound();
            self.renderer.render(self);
        } else {
            self.audio_player.play_no_rotate_sound();
        }
    }

    pub fn move_current_object_left(&mut self) {
        let new_pos = self.current_object.get_position().copy_mod_x(-1);
        if self
            .field
            .can_hold(self.current_object.get_qube_with_width(), &new_pos)
        {
            self.current_object.pos.mod_x(-1);
            self.audio_player.play_move_sound();
            self.renderer.render(self);
        } else {
            self.audio_player.play_no_move_sound();
        }
    }

    pub fn move_current_object_right(&mut self) {
        let new_pos = self.current_object.get_position().copy_mod_x(1);
        if self
            .field
            .can_hold(self.current_object.get_qube_with_width(), &new_pos)
        {
            self.current_object.pos.mod_x(1);
            self.audio_player.play_move_sound();
            self.renderer.render(self);
        } else {
            self.audio_player.play_no_move_sound();
        }
    }

    pub fn move_current_object_down(&mut self) {
        let new_pos = self.current_object.get_position().copy_mod_y(1);
        if self
            .field
            .can_hold(self.current_object.get_qube_with_width(), &new_pos)
        {
            self.current_object.pos.mod_y(1);
            self.audio_player.play_move_sound();
            self.add_score(1);
            self.renderer.render(self);
        }
    }

    pub fn drop_current_object_down(&mut self) {
        let mut skipped_lines = 0_u16;

        while self.field.can_hold(
            self.current_object.get_qube_with_width(),
            &self.current_object.get_position().copy_mod_y(1),
        ) {
            self.current_object.pos.mod_y(1);
            skipped_lines += 1;
        }

        if skipped_lines == 0 {
            // If no lines were skipped, we do not add any score
            return;
        }

        self.audio_player.play_drop_sound();

        // Place the current object in the field after dropping it down
        self.field
            .place(
                self.current_object.get_qube_with_width(),
                &self.current_object.get_position(),
            )
            .expect("Failed to place the current object in the field!");

        self.add_score((skipped_lines as f64 * 1.25) as u64);

        // Move to the next object
        self.next();
    }

    pub fn tick(&mut self) {
        if self.paused || self.game_over {
            // If the game is paused or over, we do not process the tick
            return;
        }

        let time = Utc::now().timestamp_millis();

        if self.last_tick == 0 {
            // If this is the first tick, we set the last_tick to the current time
            self.last_tick = time;
            self.renderer.render(self);
            return;
        }

        // Calculate the time difference since the last tick relative to the current level, which determines the speed of the game 1 - 255
        let time_diff = time - self.last_tick;
        if time_diff < calc_level_speed(self.level) {
            // If the time difference is less than the level speed, we do not process the tick
            return;
        }

        let completed_lines = self.field.clear_completed_lines();
        if completed_lines > 0 {
            if completed_lines == 4 {
                // If 4 lines were cleared, it is a Tetris
                self.audio_player.play_tetris_line_clear_sound();
            } else {
                self.audio_player.play_line_clear_sound();
            }

            self.add_score(10 + ((completed_lines as u64 - 1) * 15));
            self.last_tick = time + time_diff.div(2);
            self.renderer.render(self);
            return;
        }

        if self.field.can_hold(
            self.current_object.get_qube_with_width(),
            &self.current_object.get_position().copy_mod_y(1),
        ) {
            self.current_object.pos.mod_y(1);
        } else {
            // If the current object cannot move down, it means it has landed
            // We need to place it in the field and check for completed lines
            self.field
                .place(
                    self.current_object.get_qube_with_width(),
                    &self.current_object.get_position(),
                )
                .expect("Failed to place the current object in the field!");

            self.next();
        }

        self.last_tick = time;
        self.renderer.render(self);
    }
}
