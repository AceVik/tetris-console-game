use crate::game::consts::qube::Qube;
use super::{Q_1000_ROW, Q_0000_ROW, Q_1111_ROW};

pub const I_UP: Qube = [
  Q_1000_ROW,
  Q_1000_ROW,
  Q_1000_ROW,
  Q_1000_ROW,
];

pub const I_RIGHT: Qube = [
  Q_1111_ROW,
  Q_0000_ROW,
  Q_0000_ROW,
  Q_0000_ROW,
];

pub const I_DOWN: Qube = I_UP;

pub const I_LEFT: Qube = I_RIGHT;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::game::consts::qube::QUBE_SIZE;

  #[test]
  fn test_i_up() {
    assert_eq!(I_UP.len(), QUBE_SIZE as usize);
    assert_eq!(I_UP[0], 0b1000_0000);
    assert_eq!(I_UP[1], 0b1000_0000);
    assert_eq!(I_UP[2], 0b1000_0000);
    assert_eq!(I_UP[3], 0b1000_0000);
  }

  #[test]
  fn test_i_right() {
    assert_eq!(I_RIGHT.len(), QUBE_SIZE as usize);
    assert_eq!(I_RIGHT[0], 0b1111_0000);
    assert_eq!(I_RIGHT[1], 0b0000_0000);
    assert_eq!(I_RIGHT[2], 0b0000_0000);
    assert_eq!(I_RIGHT[3], 0b0000_0000);
  }

  #[test]
  fn test_i_down() {
    assert_eq!(I_DOWN.len(), QUBE_SIZE as usize);
    assert_eq!(I_DOWN[0], 0b1000_0000);
    assert_eq!(I_DOWN[1], 0b1000_0000);
    assert_eq!(I_DOWN[2], 0b1000_0000);
    assert_eq!(I_DOWN[3], 0b1000_0000);
  }

  #[test]
  fn test_i_left() {
    assert_eq!(I_LEFT.len(), QUBE_SIZE as usize);
    assert_eq!(I_LEFT[0], 0b1111_0000);
    assert_eq!(I_LEFT[1], 0b0000_0000);
    assert_eq!(I_LEFT[2], 0b0000_0000);
    assert_eq!(I_LEFT[3], 0b0000_0000);
  }
}