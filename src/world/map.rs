enum Tile {
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
pub struct Map {
    tiles: Vec<Vec<Tile>>
}

impl Map {

    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    pub fn as_string(&self) -> String {
        if self.tiles.len() == 0 { return "".to_string(); }

        let tiles_size = self.tiles.len() * self.tiles[0].len();

        let mut string_repr = String::with_capacity(tiles_size);


        for i in 0..(self.tiles.len()) {
            for j in 0..(self.tiles[0].len()) {
                let char_for_tile = match self.tiles[i][j] {
                    Tile::Blank => ' ',
                    Tile::Wall => '#'
                };
                string_repr.push(char_for_tile);
            }

            if i != self.tiles.len() - 1 {
                string_repr.push('\n');
            }
        }

        string_repr.to_string()
    }

}

// impl Default for Map {
//
// }

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
