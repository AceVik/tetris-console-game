use super::field::FIELD_WIDTH;
use std::mem::size_of;

pub const QUBE_SIZE: u16 = 4;
pub type QubeRow = u8;
#[allow(dead_code)]
pub const QUBE_ROW: QubeRow = QubeRow::MAX << QUBE_SIZE;

pub type Qube = [QubeRow; QUBE_SIZE as usize];

#[allow(dead_code)]
pub const EMPTY_QUBE: Qube = [QUBE_ROW; QUBE_SIZE as usize];

pub const QUBE_ROW_DEFAULT_X: u16 = FIELD_WIDTH - size_of::<QubeRow>() as u16 * 8;
