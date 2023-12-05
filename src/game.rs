pub const BOARD_LENGTH : usize = 3;
pub const GAME_LENGTH : usize = BOARD_LENGTH * BOARD_LENGTH;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coordinates {
    pub x : usize,
    pub y : usize,
}

impl Coordinates {

    fn to_meta(&self) -> Coordinates {
        return Coordinates {
            x: self.x / BOARD_LENGTH,
            y: self.y / BOARD_LENGTH,
        }
    }

    fn to_local(&self) -> Coordinates {
        return Coordinates {
            x: self.x % BOARD_LENGTH,
            y: self.y % BOARD_LENGTH,
        }
    }

    fn on_diagonal_00(&self) -> bool {
        return self.x == self.y;
    }

    fn on_diagonal_02(&self) -> bool {
        return match (self.x, self.y) {
            (0, 2) | (1, 1) | (2, 0) => true,
            _ => false
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Cross,
    Nought,
}

pub type Board = [[Option<Piece>; BOARD_LENGTH]; BOARD_LENGTH];


#[derive(Copy, Clone, Debug)]
pub struct GameState {
    pub meta_coords_restriction : Option<Coordinates>,
    pub meta_pieces: Board,
    pub pieces: [[Board; BOARD_LENGTH]; BOARD_LENGTH],
    pub turn: Piece,
    pub game_over: bool,
}

impl GameState {

    pub fn new() -> GameState {
        return GameState {
            meta_coords_restriction: None,
            meta_pieces: [[None; BOARD_LENGTH]; BOARD_LENGTH],
            pieces: [[[[None; BOARD_LENGTH]; BOARD_LENGTH]; BOARD_LENGTH]; BOARD_LENGTH],
            turn: Piece::Nought,
            game_over: false,
        }
    }

    pub fn request_action(&mut self, global_coords : Coordinates) {
        if self.game_over {
            println!("Game over");
            return
        }
        let meta_coords = global_coords.to_meta();
        if let Some(coords) = self.meta_coords_restriction {
            if coords != meta_coords {
                println!("Move must take place on meta-board {:?}", coords);
                return;
            }
        }
        match self.meta_pieces[meta_coords.x][meta_coords.y] {
            Some(piece) => println!("Board already won by {:?}", piece),
            None => {
                let local_coords = global_coords.to_local();
                match self.pieces[meta_coords.x][meta_coords.y][local_coords.x][local_coords.y] {
                    Some(piece) => println!("Tile already taken by {:?}", piece),
                    None => {
                        self.pieces[meta_coords.x][meta_coords.y][local_coords.x][local_coords.y] = Some(self.turn);
                        if self.move_wins_board(self.pieces[meta_coords.x][meta_coords.y], local_coords) {
                            self.meta_pieces[meta_coords.x][meta_coords.y] = Some(self.turn);
                            if self.move_wins_board(self.meta_pieces, meta_coords) {
                                println!("Game over! {:?} wins", self.turn);
                                self.game_over = true;
                                return;
                            }
                        }
                        self.meta_coords_restriction = if self.board_is_open(local_coords) { Some(local_coords) } else { None };
                        self.turn = match self.turn {
                            Piece::Nought => Piece::Cross,
                            Piece::Cross => Piece::Nought
                        }
                    }
                }
            }
        }
    }

    fn board_is_open(&self, meta_coords : Coordinates) -> bool {
        if self.meta_pieces[meta_coords.x][meta_coords.y].is_none() {
            for column in self.pieces[meta_coords.x][meta_coords.y].iter() {
                for piece_option in column.iter() {
                    if piece_option.is_none() {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    fn move_wins_board(&self, board : Board, coords : Coordinates) -> bool {
        let (mut row_win, mut column_win) = (true, true);
        let (mut diagonal_00_win, mut diagonal_02_win) = (coords.on_diagonal_00(), coords.on_diagonal_02());
        for index in 0..BOARD_LENGTH {
            if row_win {
                row_win = board[coords.x][index] == Some(self.turn);
            }
            if column_win {
                column_win = board[index][coords.y] == Some(self.turn);
            }
            if diagonal_00_win {
                diagonal_00_win = board[index][index] == Some(self.turn);
            }
            if diagonal_02_win {
                diagonal_02_win = board[index][BOARD_LENGTH - 1 - index] == Some(self.turn);
            }
        }
        return row_win || column_win || diagonal_00_win || diagonal_02_win;
    }
}