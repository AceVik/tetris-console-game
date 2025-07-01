pub trait AudioPlayer: Send + Sync {
    fn play_background_music(&self);
    fn play_game_over_sound(&self);
    fn play_pause_sound(&self);
    fn play_line_clear_sound(&self);
    fn play_tetris_line_clear_sound(&self);
    fn play_move_sound(&self);
    fn play_no_move_sound(&self);
    fn play_rotate_sound(&self);
    fn play_no_rotate_sound(&self);
    fn play_drop_sound(&self);
    fn play_level_up_sound(&self);

    fn mute_background_music(&mut self);
    #[allow(dead_code)]
    fn mute_effects(&mut self);
}
