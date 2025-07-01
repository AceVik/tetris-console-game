use crate::game::consts::qube::Qube;
use super::{Q_0000_ROW, Q_1100_ROW};

pub const O_UP: Qube = [
  Q_1100_ROW,
  Q_1100_ROW,
  Q_0000_ROW,
  Q_0000_ROW,
];

pub const O_RIGHT: Qube = O_UP;

pub const O_DOWN: Qube = O_UP;

pub const O_LEFT: Qube = O_UP;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::game::consts::qube::QUBE_SIZE;

  #[test]
  fn test_o_up() {
    assert_eq!(O_UP.len(), QUBE_SIZE as usize);
    assert_eq!(O_UP[0], 0b1100_0000);
    assert_eq!(O_UP[1], 0b1100_0000);
    assert_eq!(O_UP[2], 0b0000_0000);
    assert_eq!(O_UP[3], 0b0000_0000);
  }
  
  #[test]
  fn test_o_right() {
    assert_eq!(O_RIGHT.len(), QUBE_SIZE as usize);
    assert_eq!(O_RIGHT[0], 0b1100_0000);
    assert_eq!(O_RIGHT[1], 0b1100_0000);
    assert_eq!(O_RIGHT[2], 0b0000_0000);
    assert_eq!(O_RIGHT[3], 0b0000_0000);
  }
  
  #[test]
  fn test_o_down() {
    assert_eq!(O_DOWN.len(), QUBE_SIZE as usize);
    assert_eq!(O_DOWN[0], 0b1100_0000);
    assert_eq!(O_DOWN[1], 0b1100_0000);
    assert_eq!(O_DOWN[2], 0b0000_0000);
    assert_eq!(O_DOWN[3], 0b0000_0000);
  }
  
  #[test]
  fn test_o_left() {
    assert_eq!(O_LEFT.len(), QUBE_SIZE as usize);
    assert_eq!(O_LEFT[0], 0b1100_0000);
    assert_eq!(O_LEFT[1], 0b1100_0000);
    assert_eq!(O_LEFT[2], 0b0000_0000);
    assert_eq!(O_LEFT[3], 0b0000_0000);
  }
}