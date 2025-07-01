use crate::game::consts::qube::QubeRow;

const Q_0000_ROW: QubeRow = 0b0000_0000;
const Q_1000_ROW: QubeRow = 0b1000_0000;
const Q_1111_ROW: QubeRow = 0b1111_0000;
const Q_1110_ROW: QubeRow = 0b1110_0000;
const Q_0010_ROW: QubeRow = 0b0010_0000;
const Q_1100_ROW: QubeRow = 0b1100_0000;
const Q_0100_ROW: QubeRow = 0b0100_0000;
const Q_0110_ROW: QubeRow = 0b0110_0000;

pub const EMPTY_QUBE_ROW: QubeRow = Q_0000_ROW;

pub mod l;
pub mod j;
pub mod t;
pub mod i;
pub mod o;
pub mod s;
pub mod z;