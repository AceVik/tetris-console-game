use crate::game::consts::qube::Qube;
use super::{Q_1000_ROW, Q_0000_ROW, Q_0010_ROW, Q_1110_ROW, Q_1100_ROW, Q_0100_ROW};

pub const L_UP: Qube = [
  Q_0010_ROW,
  Q_1110_ROW,
  Q_0000_ROW,
  Q_0000_ROW,
];

pub const L_RIGHT: Qube = [
  Q_1000_ROW,
  Q_1000_ROW,
  Q_1100_ROW,
  Q_0000_ROW,
];

pub const L_DOWN: Qube = [
  Q_1110_ROW,
  Q_1000_ROW,
  Q_0000_ROW,
  Q_0000_ROW,
];

pub const L_LEFT: Qube = [
  Q_1100_ROW,
  Q_0100_ROW,
  Q_0100_ROW,
  Q_0000_ROW,
];

#[cfg(test)]
mod tests {
  use super::*;
  use crate::game::consts::qube::QUBE_SIZE;

  #[test]
  fn test_l_up() {
    assert_eq!(L_UP.len(), QUBE_SIZE as usize);
    assert_eq!(L_UP[0], 0b0010_0000);
    assert_eq!(L_UP[1], 0b1110_0000);
    assert_eq!(L_UP[2], 0b0000_0000);
    assert_eq!(L_UP[3], 0b0000_0000);
  }
  
  #[test]
  fn test_l_right() {
    assert_eq!(L_RIGHT.len(), QUBE_SIZE as usize);
    assert_eq!(L_RIGHT[0], 0b1000_0000);
    assert_eq!(L_RIGHT[1], 0b1000_0000);
    assert_eq!(L_RIGHT[2], 0b1100_0000);
    assert_eq!(L_RIGHT[3], 0b0000_0000);
  }
  
  #[test]
  fn test_l_down() {
    assert_eq!(L_DOWN.len(), QUBE_SIZE as usize);
    assert_eq!(L_DOWN[0], 0b1110_0000);
    assert_eq!(L_DOWN[1], 0b1000_0000);
    assert_eq!(L_DOWN[2], 0b0000_0000);
    assert_eq!(L_DOWN[3], 0b0000_0000);
  }
  
  #[test]
  fn test_l_left() {
    assert_eq!(L_LEFT.len(), QUBE_SIZE as usize);
    assert_eq!(L_LEFT[0], 0b1100_0000);
    assert_eq!(L_LEFT[1], 0b0100_0000);
    assert_eq!(L_LEFT[2], 0b0100_0000);
    assert_eq!(L_LEFT[3], 0b0000_0000);
  }
}

