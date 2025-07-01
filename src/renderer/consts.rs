use crossterm::style::Color;
use crate::game::consts::field::{FIELD_HEIGHT, FIELD_WIDTH};
use crate::game::consts::qube::QUBE_SIZE;

pub const EMPTY_BLOCK_COLOR: Color = Color::Grey;
pub const FILLED_BLOCK_COLOR: Color = Color::White;

pub const BORDER_HORIZONTAL: char = '═';
pub const BORDER_VERTICAL: char = '║';
pub const BORDER_TOP_LEFT: char = '╔';
pub const BORDER_TOP_RIGHT: char = '╗';
pub const BORDER_BOTTOM_LEFT: char = '╚';
pub const BORDER_BOTTOM_RIGHT: char = '╝';
pub const BORDER_TOP_MIDDLE: char = '╤';
pub const BORDER_BOTTOM_MIDDLE: char = '╧';
pub const BORDER_VERTICAL_SINGLE: char = '│';

pub const H_OFFSET: u16 = 1;
pub const V_OFFSET: u16 = 0;

pub const SIDEBAR_WIDTH: u16 = QUBE_SIZE * 2;
pub const AREA_WIDTH: u16 = FIELD_WIDTH * 2;

pub const CONTAINER_WIDTH: u16 = AREA_WIDTH + 2 + SIDEBAR_WIDTH + 1;
pub const CONTAINER_HEIGHT: u16 = FIELD_HEIGHT + 2;

pub const SCREEN_WIDTH: u16 = CONTAINER_WIDTH + H_OFFSET * 2;
pub const SCREEN_HEIGHT: u16 = CONTAINER_HEIGHT + V_OFFSET * 2;