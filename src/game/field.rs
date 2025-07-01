use super::consts::field::{
    Area, Row, EMPTY_AREA, EMPTY_ROW, FIELD_HEIGHT, FIELD_WIDTH, ROW_OFFSET,
};
use super::consts::qube::{Qube, QubeRow, QUBE_ROW_DEFAULT_X};
use super::consts::qubes::EMPTY_QUBE_ROW;
use super::geometry::{Direction, Pos};
use crate::game::tetromino::Tetromino;
use std::cmp;
use std::fmt::Display;
use std::ops::Div;

#[inline(always)]
fn get_cube_row_mask(row: &QubeRow, pos_x: u16) -> Row {
    // FRow: 0b111111_0000000000 -> 1's are ignored, 0's are the area to check
    // QRow: 0b1111_0000 -> 1's are the qube to check, 0's are ignored
    // Pow.x is the position in the field where the qube is placed -> x = 2 -> 0b111111_0011110000
    // QUBE_ROW_DEFAULT_X represents the shifted qube row bits as default position in x (u8 -> 8bits, left bits are interesting)
    (if pos_x < QUBE_ROW_DEFAULT_X {
        (*row as Row) << (QUBE_ROW_DEFAULT_X - pos_x)
    } else {
        (*row as Row) >> (pos_x - QUBE_ROW_DEFAULT_X)
    }) as Row
        | EMPTY_ROW // OR with EMPTY_ROW to set out of bounds bits to 1
}

#[allow(dead_code)]
#[inline(always)]
fn count_row_width(row: &Row) -> u16 {
    for i in (0_u8..4_u8).rev() {
        if row & (1_u16 << i) != 0 {
            return i as u16 + 1_u16; // Return the width of the row, counting from the right
        }
    }

    0
}

pub struct Field {
    pub width: u16,
    #[allow(dead_code)]
    pub height: u16,
    pub area: Area,
}

impl Field {
    pub fn new() -> Self {
        Field {
            width: FIELD_WIDTH,
            height: FIELD_HEIGHT,
            area: EMPTY_AREA,
        }
    }

    pub fn reset(&mut self) {
        self.area = EMPTY_AREA;
    }

    #[inline(always)]
    pub fn get_start_pos(&self, tetromino: &Tetromino, direction: &Direction) -> Option<Pos> {
        let (width, _) = tetromino.dimensions(&direction);
        let initial_x = (self.width - width).div(2); // Center the tetromino in the field, considering its width
        let mut pos = Pos::new(initial_x, 0).unwrap_or(Pos::zero());
        let qube = tetromino.get_cube_by_direction(&direction);

        if self.can_hold((&qube, width), &pos) {
            return Some(pos);
        }

        for i in 2..=self.width {
            pos.x = if i % 2 == 0 {
                initial_x + i.div(2)
            } else {
                cmp::max(0, initial_x as i32 - i.div(2) as i32) as u16
            };

            if self.can_hold((&qube, width), &pos) {
                return Some(pos);
            }
        }

        None
    }

    #[inline(always)]
    pub fn can_hold(&self, (qube, width): (&Qube, u16), pos: &Pos) -> bool {
        if width + pos.x > self.width {
            // If the qube width + position x exceeds the field width, it cannot be held
            return false;
        }

        for (y, row) in qube
            .iter()
            .filter(|&&row| row != EMPTY_QUBE_ROW)
            .enumerate()
        {
            let qube_row_mask = get_cube_row_mask(row, pos.x);

            match self.area.get(pos.y as usize + y) {
                Some(&field_row) => {
                    if (field_row & qube_row_mask) != EMPTY_ROW {
                        // If the field row AND the qube row mask is not empty, it means there is a collision
                        return false;
                    }
                }

                None => {
                    // If the position is out of bounds, we cannot hold the qube
                    return false;
                }
            }
        }

        // If we reach here, it means the qube can be held in the field at the given position
        true
    }

    pub fn place(&mut self, qube_with_width: (&Qube, u16), pos: &Pos) -> Result<(), &'static str> {
        if !self.can_hold(qube_with_width, pos) {
            return Err("Cannot place qube at the given position");
        }

        for (y, row) in qube_with_width
            .0
            .iter()
            .filter(|&&row| row != EMPTY_QUBE_ROW)
            .enumerate()
        {
            let qube_row_mask = get_cube_row_mask(row, pos.x);
            let field_row = self.area.get_mut(pos.y as usize + y).unwrap();
            *field_row |= qube_row_mask; // Set the bits in the field row to 1 where the qube is placed
        }

        Ok(())
    }

    /// Clears completed lines from the field and returns the row index of the last cleared line and the number of cleared lines.
    #[inline(always)]
    fn get_completed_rows(&mut self) -> (u16, u16) {
        let mut cleared_lines = 0_u16;

        for (y, row) in self.area.iter_mut().enumerate() {
            if *row == Row::MAX {
                cleared_lines += 1;
            } else if cleared_lines > 0 {
                return (y as u16 - 1, cleared_lines);
            }
        }

        (self.height - 1, cleared_lines)
    }

    #[inline(always)]
    fn move_all_rows_down(&mut self, (last_row, rows): (u16, u16)) {
        for y in (0..=last_row).rev() {
            if y < rows {
                break;
            }

            self.area[y as usize] = self.area[(y - rows) as usize]; // Move rows down
        }

        for y in 0..rows {
            self.area[y as usize] = EMPTY_ROW;
        }
    }

    pub fn clear_completed_lines(&mut self) -> u16 {
        let (last_cleared_line, cleared_lines) = self.get_completed_rows();
        if cleared_lines > 0 {
            self.move_all_rows_down((last_cleared_line, cleared_lines));
        }

        cleared_lines
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.area.iter() {
            write!(
                f,
                "{:0>width$b}\n",
                row.clone() << ROW_OFFSET,
                width = self.width as usize
            )?;
        }

        Ok(())
    }
}

// Tests generated by Junie AI
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::geometry::{Direction, Pos};
    use crate::game::tetromino::Tetromino;

    #[test]
    fn test_new_field() {
        let field = Field::new();

        assert_eq!(field.width, FIELD_WIDTH);
        assert_eq!(field.height, FIELD_HEIGHT);
        assert_eq!(field.area, EMPTY_AREA);
    }

    #[test]
    fn test_reset() {
        let mut field = Field::new();

        // Modify the field
        field.area[0] = 0xFFFF;

        // Reset the field
        field.reset();

        // Check that the field is empty
        assert_eq!(field.area, EMPTY_AREA);
    }

    #[test]
    fn test_get_start_pos() {
        let field = Field::new();
        let tetromino = Tetromino::L;
        let direction = Direction::Up;

        let pos = field.get_start_pos(&tetromino, &direction);

        // The start position should be valid
        assert!(pos.is_some());

        // The start position should be centered horizontally
        let pos = pos.unwrap();
        let (width, _) = tetromino.dimensions(&direction);
        assert_eq!(pos.x, (field.width - width) / 2);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn test_can_hold() {
        let field = Field::new();
        let tetromino = Tetromino::L;
        let direction = Direction::Up;
        let qube = tetromino.get_cube_by_direction(&direction);
        let width = tetromino.dimensions(&direction).0;

        // Position at the top center of the field
        let pos = Pos::new((field.width - width) / 2, 0).unwrap();

        // The field should be able to hold the tetromino at the start position
        assert!(field.can_hold((&qube, width), &pos));

        // Position outside the field horizontally
        let pos = Pos::new(field.width - width + 1, 0).unwrap();

        // The field should not be able to hold the tetromino outside the field
        assert!(!field.can_hold((&qube, width), &pos));

        // Test with a position that would place part of the tetromino outside the field vertically
        // We need a valid position that's close to the bottom of the field
        let pos = Pos::new(0, field.height - 1).unwrap();

        // The field should not be able to hold the tetromino if it would extend beyond the field
        // This is because the tetromino's height is greater than 1, so it would extend beyond the field
        assert!(!field.can_hold((&qube, width), &pos));
    }

    #[test]
    fn test_place() {
        let mut field = Field::new();
        let tetromino = Tetromino::L;
        let direction = Direction::Up;
        let qube = tetromino.get_cube_by_direction(&direction);
        let width = tetromino.dimensions(&direction).0;

        // Position at the top center of the field
        let pos = Pos::new((field.width - width) / 2, 0).unwrap();

        // Place the tetromino
        let result = field.place((&qube, width), &pos);

        // The placement should succeed
        assert!(result.is_ok());

        // The field should no longer be empty
        assert_ne!(field.area, EMPTY_AREA);

        // Try to place the tetromino at the same position again
        let result = field.place((&qube, width), &pos);

        // The placement should fail because the position is already occupied
        assert!(result.is_err());
    }

    #[test]
    fn test_clear_completed_lines() {
        let mut field = Field::new();

        // No completed lines initially
        assert_eq!(field.clear_completed_lines(), 0);

        // Fill a line
        field.area[5] = Row::MAX;

        // Clear the completed line
        assert_eq!(field.clear_completed_lines(), 1);

        // The field should be empty again
        assert_eq!(field.area[5], EMPTY_ROW);

        // Fill multiple lines
        field.area[5] = Row::MAX;
        field.area[6] = Row::MAX;
        field.area[7] = Row::MAX;

        // Clear the completed lines
        assert_eq!(field.clear_completed_lines(), 3);

        // The field should be empty again
        assert_eq!(field.area[5], EMPTY_ROW);
        assert_eq!(field.area[6], EMPTY_ROW);
        assert_eq!(field.area[7], EMPTY_ROW);
    }
}
