use super::consts::qube::Qube;
use super::consts::qubes::i::{I_DOWN, I_LEFT, I_RIGHT, I_UP};
use super::consts::qubes::j::{J_DOWN, J_LEFT, J_RIGHT, J_UP};
use super::consts::qubes::l::{L_DOWN, L_LEFT, L_RIGHT, L_UP};
use super::consts::qubes::o::{O_DOWN, O_LEFT, O_RIGHT, O_UP};
use super::consts::qubes::s::{S_DOWN, S_LEFT, S_RIGHT, S_UP};
use super::consts::qubes::t::{T_DOWN, T_LEFT, T_RIGHT, T_UP};
use super::consts::qubes::z::{Z_DOWN, Z_LEFT, Z_RIGHT, Z_UP};
use super::geometry::Direction;
use rand::{rng, Rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tetromino {
    L,
    J,
    T,
    I,
    O,
    S,
    Z,
}

impl Tetromino {
    pub fn dimensions(&self, direction: &Direction) -> (u16, u16) {
        match self {
            Tetromino::L | Tetromino::J | Tetromino::T => match direction {
                Direction::Up | Direction::Down => (3, 2),
                Direction::Left | Direction::Right => (2, 3),
            },
            Tetromino::I => match direction {
                Direction::Up | Direction::Down => (1, 4),
                Direction::Left | Direction::Right => (4, 1),
            },
            Tetromino::O => (2, 2),
            Tetromino::S | Tetromino::Z => match direction {
                Direction::Up | Direction::Down => (3, 2),
                Direction::Left | Direction::Right => (2, 3),
            },
        }
    }

    #[allow(dead_code)]
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        match rng.random_range(0..7) {
            0 => Tetromino::L,
            1 => Tetromino::J,
            2 => Tetromino::T,
            3 => Tetromino::I,
            4 => Tetromino::O,
            5 => Tetromino::S,
            _ => Tetromino::Z,
        }
    }

    pub fn get_cube_by_direction(&self, direction: &Direction) -> Qube {
        match self {
            Tetromino::L => match direction {
                Direction::Up => L_UP,
                Direction::Down => L_DOWN,
                Direction::Left => L_LEFT,
                Direction::Right => L_RIGHT,
            },
            Tetromino::J => match direction {
                Direction::Up => J_UP,
                Direction::Down => J_DOWN,
                Direction::Left => J_LEFT,
                Direction::Right => J_RIGHT,
            },
            Tetromino::T => match direction {
                Direction::Up => T_UP,
                Direction::Down => T_DOWN,
                Direction::Left => T_LEFT,
                Direction::Right => T_RIGHT,
            },
            Tetromino::I => match direction {
                Direction::Up => I_UP,
                Direction::Down => I_DOWN,
                Direction::Left => I_LEFT,
                Direction::Right => I_RIGHT,
            },
            Tetromino::O => match direction {
                Direction::Up => O_UP,
                Direction::Down => O_DOWN,
                Direction::Left => O_LEFT,
                Direction::Right => O_RIGHT,
            },
            Tetromino::S => match direction {
                Direction::Up => S_UP,
                Direction::Down => S_DOWN,
                Direction::Left => S_LEFT,
                Direction::Right => S_RIGHT,
            },
            Tetromino::Z => match direction {
                Direction::Up => Z_UP,
                Direction::Down => Z_DOWN,
                Direction::Left => Z_LEFT,
                Direction::Right => Z_RIGHT,
            },
        }
    }
}

pub struct TetrominoBag {
    weights: [u32; 7],
}

impl TetrominoBag {
    pub fn new() -> Self {
        TetrominoBag { weights: [1; 7] }
    }

    fn tetromino_to_idx(tetromino: &Tetromino) -> usize {
        match tetromino {
            Tetromino::L => 0,
            Tetromino::J => 1,
            Tetromino::T => 2,
            Tetromino::I => 3,
            Tetromino::O => 4,
            Tetromino::S => 5,
            Tetromino::Z => 6,
        }
    }

    fn idx_to_tetromino(idx: usize) -> Tetromino {
        match idx {
            0 => Tetromino::L,
            1 => Tetromino::J,
            2 => Tetromino::T,
            3 => Tetromino::I,
            4 => Tetromino::O,
            5 => Tetromino::S,
            6 => Tetromino::Z,
            _ => unreachable!("Invalid Tetromino index"),
        }
    }

    pub fn get_next_tetromino(&self) -> Tetromino {
        let mut rng = rng();

        let total_weight: u32 = self.weights.iter().sum();
        let mut random_value = rng.random_range(0..total_weight);

        for (idx, &weight) in self.weights.iter().enumerate() {
            if random_value < weight {
                return Self::idx_to_tetromino(idx);
            }
            random_value -= weight;
        }
        unreachable!("Failed to select a tetromino from the bag.");
    }

    pub fn update_weights(&mut self, last_drawn_tetromino: &Tetromino) {
        let last_drawn_idx = Self::tetromino_to_idx(last_drawn_tetromino);

        for i in 0..self.weights.len() {
            if i == last_drawn_idx {
                self.weights[i] = 1;
            } else {
                self.weights[i] += 1;
            }
        }
    }

    pub fn get_next_tetromino_and_update_weights(&mut self) -> Tetromino {
        let next_tetromino = self.get_next_tetromino();
        self.update_weights(&next_tetromino);
        next_tetromino
    }

    pub fn reset_weights(&mut self) {
        self.weights = [1; 7];
    }
}
