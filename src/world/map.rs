use std::ops::Index;

use crate::utility::conversions;

pub enum Tile {
    Blank,
    Wall
}

impl Clone for Tile {
    fn clone(&self) -> Tile {
        match self {
            Self::Blank => Self::Blank,
            Self::Wall => Self::Wall
        }
    }
}

#[derive(Default)]
/// Map of the game world
pub struct Map {
    tiles: Vec<Vec<Tile>>
}

impl Index<usize> for Map {

    type Output = Vec<Tile>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }

}

impl Map {

    pub fn dimensions(&self) -> (usize, usize) {
        if self.tiles.len() > 0 {
            (self.tiles.len(), self.tiles[0].len())
        } else {
            (0, 0)
        }
    }

    /// Returns `true` if (x, y) is in bounds of `self.tiles`
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        if self.tiles.len() > 0 {
            let width = conversions::as_i32(self.tiles.len());
            let height = conversions::as_i32(self.tiles[0].len());

            (x >= 0 && x < width) && (y >= 0 && y < height)
        } else {
            false
        }
    }

}

pub fn test_room() -> Map {
    let mut tiles = vec![vec![Tile::Blank; 10]; 10];

    // Create 6x6 room in middle
    // \ 0 1 2 3 4 5 6 7 8 9
    // 0 . . . . . . . . . .
    // 1 . . . . . . . . . .
    // 2 . . x . . . . x . .
    // 3 . . x . . . . x . .
    // 4 . . x . . . . x . .
    // 5 . . x . . . . x . .
    // 6 . . x . . . . x . .
    // 7 . . x x x x x x . .
    // 8 . . . . . . . . . .
    // 9 . . . . . . . . . .

    tiles[2][2] = Tile::Wall;

    tiles[3][2] = Tile::Wall;
    tiles[4][2] = Tile::Wall;
    tiles[5][2] = Tile::Wall;
    tiles[6][2] = Tile::Wall;
    tiles[7][2] = Tile::Wall;

    tiles[7][3] = Tile::Wall;
    tiles[7][4] = Tile::Wall;
    tiles[7][5] = Tile::Wall;
    tiles[7][6] = Tile::Wall;
    tiles[7][7] = Tile::Wall;

    tiles[6][7] = Tile::Wall;
    tiles[5][7] = Tile::Wall;
    tiles[4][7] = Tile::Wall;
    tiles[3][7] = Tile::Wall;
    tiles[2][7] = Tile::Wall;

    tiles[2][6] = Tile::Wall;
    tiles[2][5] = Tile::Wall;
    tiles[2][4] = Tile::Wall;
    tiles[2][3] = Tile::Wall;

    Map { tiles }
}
