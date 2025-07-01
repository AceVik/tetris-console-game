use crate::game::consts::qube::Qube;
use super::{Q_0000_ROW, Q_0100_ROW, Q_0110_ROW, Q_1000_ROW, Q_1100_ROW};

pub const Z_UP: Qube = [
  Q_1100_ROW,
  Q_0110_ROW,
  Q_0000_ROW,
  Q_0000_ROW,
];

pub const Z_RIGHT: Qube = [
  Q_0100_ROW,
  Q_1100_ROW,
  Q_1000_ROW,
  Q_0000_ROW,
];

pub const Z_DOWN: Qube = Z_UP;

pub const Z_LEFT: Qube = Z_RIGHT;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::game::consts::qube::QUBE_SIZE;

  #[test]
  fn test_z_up() {
    assert_eq!(Z_UP.len(), QUBE_SIZE as usize);
    assert_eq!(Z_UP[0], 0b1100_0000);
    assert_eq!(Z_UP[1], 0b0110_0000);
    assert_eq!(Z_UP[2], 0b0000_0000);
    assert_eq!(Z_UP[3], 0b0000_0000);
  }

  #[test]
  fn test_z_right() {
    assert_eq!(Z_RIGHT.len(), QUBE_SIZE as usize);
    assert_eq!(Z_RIGHT[0], 0b0100_0000);
    assert_eq!(Z_RIGHT[1], 0b1100_0000);
    assert_eq!(Z_RIGHT[2], 0b1000_0000);
    assert_eq!(Z_RIGHT[3], 0b0000_0000);
  }
  
  #[test]
  fn test_z_down() {
    assert_eq!(Z_DOWN.len(), QUBE_SIZE as usize);
    assert_eq!(Z_DOWN[0], 0b1100_0000);
    assert_eq!(Z_DOWN[1], 0b0110_0000);
    assert_eq!(Z_DOWN[2], 0b0000_0000);
    assert_eq!(Z_DOWN[3], 0b0000_0000);
  }
  
  #[test]
  fn test_z_left() {
    assert_eq!(Z_LEFT.len(), QUBE_SIZE as usize);
    assert_eq!(Z_LEFT[0], 0b0100_0000);
    assert_eq!(Z_LEFT[1], 0b1100_0000);
    assert_eq!(Z_LEFT[2], 0b1000_0000);
    assert_eq!(Z_LEFT[3], 0b0000_0000);
  }
}