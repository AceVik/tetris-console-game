use crate::game::consts::qube::Qube;
use super::{Q_0000_ROW, Q_0010_ROW, Q_0100_ROW, Q_1000_ROW, Q_1100_ROW, Q_1110_ROW};

pub const J_UP: Qube = [
  Q_1000_ROW,
  Q_1110_ROW,
  Q_0000_ROW,
  Q_0000_ROW,
];

pub const J_RIGHT: Qube = [
  Q_1100_ROW,
  Q_1000_ROW,
  Q_1000_ROW,
  Q_0000_ROW,
];

pub const J_DOWN: Qube = [
  Q_1110_ROW,
  Q_0010_ROW,
  Q_0000_ROW,
  Q_0000_ROW,
];

pub const J_LEFT: Qube = [
  Q_0100_ROW,
  Q_0100_ROW,
  Q_1100_ROW,
  Q_0000_ROW,
];

#[cfg(test)]
mod tests {
  use super::*;
  use crate::game::consts::qube::QUBE_SIZE;

  #[test]
  fn test_j_up() {
    assert_eq!(J_UP.len(), QUBE_SIZE as usize);
    assert_eq!(J_UP[0], 0b1000_0000);
    assert_eq!(J_UP[1], 0b1110_0000);
    assert_eq!(J_UP[2], 0b0000_0000);
    assert_eq!(J_UP[3], 0b0000_0000);
  }

  #[test]
  fn test_j_right() {
    assert_eq!(J_RIGHT.len(), QUBE_SIZE as usize);
    assert_eq!(J_RIGHT[0], 0b1100_0000);
    assert_eq!(J_RIGHT[1], 0b1000_0000);
    assert_eq!(J_RIGHT[2], 0b1000_0000);
    assert_eq!(J_RIGHT[3], 0b0000_0000);
  }

  #[test]
  fn test_j_down() {
    assert_eq!(J_DOWN.len(), QUBE_SIZE as usize);
    assert_eq!(J_DOWN[0], 0b1110_0000);
    assert_eq!(J_DOWN[1], 0b0010_0000);
    assert_eq!(J_DOWN[2], 0b0000_0000);
    assert_eq!(J_DOWN[3], 0b0000_0000);
  }

  #[test]
  fn test_j_left() {
    assert_eq!(J_LEFT.len(), QUBE_SIZE as usize);
    assert_eq!(J_LEFT[0], 0b0100_0000);
    assert_eq!(J_LEFT[1], 0b0100_0000);
    assert_eq!(J_LEFT[2], 0b1100_0000);
    assert_eq!(J_LEFT[3], 0b0000_0000);
  }
}