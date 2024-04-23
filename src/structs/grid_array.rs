use std::collections::VecDeque;
use std::fmt::Debug;

use super::quater_turn::QuarterTurn;
use crate::constants::{GRID_HEIGHT, GRID_WIDTH};

// grid : GridArray<T> は [0][0] から [GRID_WIDTH - 1][GRID_HEIGHT - 1] までの成分を持つ
pub struct GridArray<T>([[T; GRID_HEIGHT]; GRID_WIDTH]);

impl<T: Copy> GridArray<T> {
    pub fn new(a: T) -> Self {
        return GridArray([[a; GRID_HEIGHT]; GRID_WIDTH]);
    }
}

impl<T: Debug> Debug for GridArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GridArray").field(&self.0).finish()
    }
}

pub struct RotatedGridArray<T> {
    grid_array: GridArray<T>,
    turn: QuarterTurn,
}

impl<T: Debug> Debug for RotatedGridArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RotatedGridArray")
            .field("grid_array", &self.grid_array)
            .field("turn", &self.turn)
            .finish()
    }
}

impl<T> RotatedGridArray<T> {
    pub fn new(grid_array: GridArray<T>, turn: QuarterTurn) -> Self {
        Self { grid_array, turn }
    }
    pub fn width(&self) -> usize {
        match self.turn {
            QuarterTurn::Deg000 | QuarterTurn::Deg180 => return GRID_WIDTH,
            QuarterTurn::Deg090 | QuarterTurn::Deg270 => return GRID_HEIGHT,
        }
    }
    pub fn height(&self) -> usize {
        match self.turn {
            QuarterTurn::Deg000 | QuarterTurn::Deg180 => return GRID_HEIGHT,
            QuarterTurn::Deg090 | QuarterTurn::Deg270 => return GRID_WIDTH,
        }
    }
    fn get(&self, i: usize, j: usize) -> Option<&T> {
        let get2 = |i, j| {
            self.grid_array
                .0
                .get(i)
                .and_then(|y_axis: &[T; GRID_HEIGHT]| (*y_axis).get(j))
        };
        match self.turn {
            QuarterTurn::Deg000 => return get2(i, j),
            QuarterTurn::Deg090 => return get2(j, GRID_HEIGHT - 1 - i),
            QuarterTurn::Deg180 => return get2(GRID_WIDTH - 1 - i, GRID_HEIGHT - 1 - j),
            QuarterTurn::Deg270 => return get2(GRID_WIDTH - 1 - j, i),
        }
    }
}

impl<T: Clone + Debug> From<RotatedGridArray<T>> for Vec<Vec<T>> {
    fn from(grid: RotatedGridArray<T>) -> Self {
        let mut vec: Vec<Vec<T>> = Vec::new();
        for i in 0..grid.width() {
            let mut y_axis: Vec<T> = Vec::new();
            for j in 0..grid.height() {
                let op = grid.get(i, j);
                let val = op.unwrap().clone();
                y_axis.push(val);
            }
            vec.push(y_axis);
        }
        return vec;
    }
}

impl<T: Clone + Debug> From<RotatedGridArray<T>> for VecDeque<VecDeque<T>> {
    fn from(val: RotatedGridArray<T>) -> Self {
        let vec_vec: Vec<Vec<T>> = val.into();
        return vec_vec.into_iter().map(|vec| vec.into()).collect();
    }
}
