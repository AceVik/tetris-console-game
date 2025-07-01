use crate::game::consts::qube::Qube;
use super::{Q_0000_ROW, Q_0100_ROW, Q_0110_ROW, Q_1000_ROW, Q_1100_ROW};

pub const S_UP: Qube = [
  Q_0110_ROW,
  Q_1100_ROW,
  Q_0000_ROW,
  Q_0000_ROW,
];

pub const S_RIGHT: Qube = [
  Q_1000_ROW,
  Q_1100_ROW,
  Q_0100_ROW,
  Q_0000_ROW,
];

pub const S_DOWN: Qube = S_UP;

pub const S_LEFT: Qube = S_RIGHT;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::game::consts::qube::QUBE_SIZE;

  #[test]
  fn test_s_up() {
    assert_eq!(S_UP.len(), QUBE_SIZE as usize);
    assert_eq!(S_UP[0], 0b0110_0000);
    assert_eq!(S_UP[1], 0b1100_0000);
    assert_eq!(S_UP[2], 0b0000_0000);
    assert_eq!(S_UP[3], 0b0000_0000);
  }

  #[test]
  fn test_s_right() {
    assert_eq!(S_RIGHT.len(), QUBE_SIZE as usize);
    assert_eq!(S_RIGHT[0], 0b1000_0000);
    assert_eq!(S_RIGHT[1], 0b1100_0000);
    assert_eq!(S_RIGHT[2], 0b0100_0000);
    assert_eq!(S_RIGHT[3], 0b0000_0000);
  }
  
  #[test]
  fn test_s_down() {
    assert_eq!(S_DOWN.len(), QUBE_SIZE as usize);
    assert_eq!(S_DOWN[0], 0b0110_0000);
    assert_eq!(S_DOWN[1], 0b1100_0000);
    assert_eq!(S_DOWN[2], 0b0000_0000);
    assert_eq!(S_DOWN[3], 0b0000_0000);
  }
  
  #[test]
  fn test_s_left() {
    assert_eq!(S_LEFT.len(), QUBE_SIZE as usize);
    assert_eq!(S_LEFT[0], 0b1000_0000);
    assert_eq!(S_LEFT[1], 0b1100_0000);
    assert_eq!(S_LEFT[2], 0b0100_0000);
    assert_eq!(S_LEFT[3], 0b0000_0000);
  }
}