use bevy::prelude::*;
use rand::Rng;

#[derive(Debug, PartialEq, Resource, Default, Copy, Clone)]
pub struct TicTacToe {
    board: [[Option<char>; 3]; 3],
    current_player: Player,
}

#[derive(Debug, PartialEq, Component, Copy, Clone, Default)]
pub enum Player {
    #[default] X,
    O,
}

#[derive(Debug, PartialEq, Component, Copy, Clone, Default)]
pub enum GameResult {
    X,
    O,
    TieX,
    TieO,
    #[default] InProgressX,
    InProgressO,
}

impl GameResult {
    pub fn from_winning_char(c: char) -> GameResult {
        match c {
            'X' => GameResult::X,
            'O' => GameResult::O,
            _ => panic!("Invalid winning char"),
        }
    }
}

impl Into<u8> for GameResult {
    fn into(self) -> u8 {
        match self{
            GameResult::InProgressX => 0b000,
            GameResult::InProgressO => 0b100,
            GameResult::X => 0b001,
            GameResult::O => 0b010,
            GameResult::TieX => 0b011,
            GameResult::TieO => 0b111,
        }
    }
}

impl TryFrom<u8> for GameResult {
    type Error = &'static str;
    fn try_from(packed: u8) -> Result<GameResult, Self::Error>{
        match packed {
            0b000 => Ok(GameResult::InProgressX),
            0b100 => Ok(GameResult::InProgressO), 
            0b001 => Ok(GameResult::X),
            0b010 => Ok(GameResult::O),
            0b011 => Ok(GameResult::TieX),
            0b111 => Ok(GameResult::TieO),
            _ => Err("Invalid game result"),
        }
    }
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }

    pub fn from_char(c: char) -> Player {
        match c{
            'X' => Player::X,
            'O' => Player::O,
            _ => panic!("Invalid player char"),
        }
    }
}


impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            board: [[None; 3]; 3],
            current_player: Player::X,
        }
    }
    pub fn reset(&mut self) {
        self.board = [[None; 3]; 3];
        self.current_player = Player::X;
    }

    pub fn set_board_state(&mut self, board: [[Option<char>; 3]; 3]) {
        self.board = board;
    }

    pub fn set_player(&mut self, player: Player) {
        self.current_player = player;
    }

    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if self.board[row][col].is_some() {
            return Err("That space is already taken");
        }

        self.board[row][col] = Some(self.current_player.to_char());

        self.current_player = self.current_player.other();
        

        Ok(())
    }

    pub fn game_result(&self) -> GameResult {
        // Check rows
        for row in 0..3 {
            if self.board[row][0] == self.board[row][1] && self.board[row][1] == self.board[row][2] {
                if self.board[row][0].is_some() {
                    return GameResult::from_winning_char(self.board[row][0].unwrap());
                }
            }
        }

        // Check columns
        for col in 0..3 {
            if self.board[0][col] == self.board[1][col] && self.board[1][col] == self.board[2][col] {
                if self.board[0][col].is_some() {
                    return GameResult::from_winning_char(self.board[0][col].unwrap());
                }
            }
        }

        // Check diagonals
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            if self.board[0][0].is_some() {
                return GameResult::from_winning_char(self.board[0][0].unwrap());
            }
        }

        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            if self.board[0][2].is_some() {
                return GameResult::from_winning_char(self.board[0][2].unwrap());
            }
        }

        // Check for tie
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col].is_none() {
                    if self.current_player == Player::X {
                        return GameResult::InProgressX;
                    } else {
                        return GameResult::InProgressO;
                    }
                }
            }
        }
        if self.current_player == Player::X {
            return GameResult::TieX;
        } else {
            return GameResult::TieO;
        }
    }

    

    pub fn pretty_print(&self) {
        let board = self.get_board_state();

        println!("---+---+---");
        for row in board.iter() {
            for cell in row.iter() {
                match cell {
                    Some(player) => print!(" {} ", player),
                    None => print!("   "),
                }
                print!("|");
            }
            println!();
            println!("---+---+---");
        }
        println!()
    }

    pub fn is_game_over(&self) -> bool {
        self.game_result() != GameResult::InProgressX && self.game_result() != GameResult::InProgressO
       
    }
    pub fn get_current_player(&self) -> Player {
        self.current_player
    }

    pub fn get_board_state(&self) -> [[Option<char>; 3]; 3] {
        self.board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // X |   |  
    //---+---+---
    //   | O |  
    //---+---+---
    //   |   | X
    #[test]
    fn test_make_move() {
        let mut game = TicTacToe::new();
        assert!(game.make_move(0, 0).is_ok());
        assert!(game.make_move(0, 0).is_err());
        assert!(game.make_move(0, 1).is_ok());
        assert!(game.make_move(1, 1).is_ok());
        assert!(game.make_move(2, 2).is_ok());
        assert!(game.make_move(2, 2).is_err());
    }
#[test]
fn test_is_game_over() {
    let mut game = TicTacToe::new();
    assert!(!game.is_game_over());
    // X |   |  
    //---+---+---
    //   |   |  
    //---+---+---
    //   |   | 
 
    game.make_move(0, 0).unwrap();
    // X | O |  
    //---+---+---
    //   |   |  
    //---+---+---
    //   |   | 
    game.make_move(0, 1).unwrap();
    // X | O |  
    //---+---+---
    //   | X |  
    //---+---+---
    //   |   | 
    game.make_move(1, 1).unwrap();
    // X | O |  
    //---+---+---
    //   | X |  
    //---+---+---
    //   |   | O
    game.make_move(2, 2).unwrap();
    // X | O |  
    //---+---+---
    //   | X |  
    //---+---+---
    // X |   | O
    game.make_move(2, 0).unwrap();
    println!("{:?}", game.get_board_state());
    assert!(!game.is_game_over());
    game.make_move(2, 1).unwrap();
    game.make_move(1, 0).unwrap();
    assert!(game.is_game_over());
    let mut game = TicTacToe::new();
    game.make_move(0, 0).unwrap();
    game.make_move(0, 1).unwrap();
    game.make_move(0, 2).unwrap();
    game.make_move(1, 0).unwrap();
    game.make_move(1, 1).unwrap();
    game.make_move(1, 2).unwrap();
    game.make_move(2, 0).unwrap();
    game.make_move(2, 1).unwrap();
    game.make_move(2, 2).unwrap();
    println!("{:?}", game.get_board_state());
    assert!(game.is_game_over());
}
}

pub trait TicTacToeAI
{
    fn get_move(&self, game: &TicTacToe) -> (usize, usize);
    fn update(&self, game: &TicTacToe, move_list: &Vec<((usize, usize), Player)>, win: bool);
}   



fn minimax(board: &[[Option<char>; 3]; 3], player: Player) -> (i32, Option<(usize, usize)>) {
    let mut best_score = if player == Player::X { -100 } else { 100 };
    let mut best_move = None;

    for row in 0..3 {
        for col in 0..3 {
            if board[row][col].is_none() {
                let mut new_board = *board;
                new_board[row][col] = Some(player.to_char());

                let score = if check_win(&new_board) {
                    if player == Player::X { 1 } else { -1 }
                } else {
                    let (child_score, _) = minimax(&new_board, player.other());
                    -child_score
                };

                if (player == Player::X && score > best_score) || (player == Player::O && score < best_score) {
                    best_score = score;
                    best_move = Some((row, col));
                }
            }
        }
    }

    (best_score, best_move)
}

fn check_win(board: &[[Option<char>; 3]; 3]) -> bool {
    // Check rows
    for row in 0..3 {
        if board[row][0] == board[row][1] && board[row][1] == board[row][2] && board[row][0].is_some() {
            return true;
        }
    }

    // Check columns
    for col in 0..3 {
        if board[0][col] == board[1][col] && board[1][col] == board[2][col] && board[0][col].is_some() {
            return true;
        }
    }

    // Check diagonals
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0].is_some() {
        return true;
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2].is_some() {
        return true;
    }

    false
}

pub fn get_best_move(board: &[[Option<char>; 3]; 3], player: Player) -> Option<(usize, usize)> {
    minimax(board, player).1
}

#[derive(Default)]
pub struct MinimaxAI;

impl TicTacToeAI for MinimaxAI {
    fn get_move(&self, game: &TicTacToe) -> (usize, usize) {
        let board = game.get_board_state();
        let player = game.get_current_player();
        get_best_move(&board, player).unwrap()
    }

    fn update(&self, game: &TicTacToe, move_list: &Vec<((usize, usize), Player)>, win: bool) {
        // Do nothing
    }
}

#[derive(Default)]
pub struct RandomAI;

impl TicTacToeAI for RandomAI {
    fn get_move(&self, game: &TicTacToe) -> (usize, usize) {
        let our_game = &mut game.clone();

        let mut rng = rand::thread_rng();
        let mut row = rng.gen_range(0..3);
        let mut col = rng.gen_range(0..3);
        while our_game.make_move(row, col).is_err() {
            row = rng.gen_range(0..3);
            col = rng.gen_range(0..3);
        }
        (row, col)
    }
    fn update(&self, game: &TicTacToe, move_list: &Vec<((usize, usize), Player)>, win: bool) {
        
    }
} 


impl From<TicTacToe> for u32 {
    fn from(game: TicTacToe) -> u32{
        let mut packed :u32 = 0;
        let result: u8= game.game_result().into();
        packed |= u32::from(result);
        for row in game.get_board_state().iter() {
            for cell in row.iter() {
                packed <<= 2;
                match cell {
                    Some('X') => packed |= 0b01,
                    Some('O') => packed |= 0b10,
                    None => packed |= 0b00,
                    _ => panic!("Invalid cell"),
                }
            }
        }

       return packed
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PackedGame{
    game: TicTacToe,
    result: GameResult
}

impl TryFrom<u32> for PackedGame{
    type Error = &'static str;
    fn try_from(packed: u32) -> Result<PackedGame, Self::Error>{
        let mut board = [[None; 3]; 3];
        let mut p = packed;


        for row in (0..3).rev() {
            for col in (0..3).rev() {
                let cell = match p & 0b11 {
                    0b00 => Ok(None),
                    0b01 => Ok(Some('X')),
                    0b10 => Ok(Some('O')),
                    _ => Err("Invalid cell"),
                };
                if cell.is_err() {
                    return Err("Invalid cell");
                } else {
                    board[row][col] = cell.unwrap();
                }
                p >>= 2;
            }
        }

        let maybe_result = GameResult::try_from((p & 0b0111) as u8);
        if maybe_result.is_err() {
            return Err("Invalid result");
        }
        let result = maybe_result.unwrap();

        let current_player = match result {
            GameResult::X => Player::X,
            GameResult::O => Player::O,
            GameResult::TieO => Player::O,
            GameResult::TieX => Player::X,
            GameResult::InProgressX => Player::X,
            GameResult::InProgressO => Player::O,
        };
        let mut game= TicTacToe::new();
        game.set_board_state(board);
        game.set_player(current_player);
        if game.game_result() != result {
            return Err("Result and encoded game do not match");
        }
        return Ok(PackedGame{
            game, 
            result
        });
    }
}

impl PackedGame{
    pub fn get_game(&self) -> &TicTacToe{
        return &self.game;
    }
    pub fn get_result(&self) -> GameResult{
        return self.result;
    }
}  



#[test]
fn test_packed_game_encoding() {
    for i  in 0..=0b111111111111111111111 {
        let maybe_game:Result<PackedGame, &str> = PackedGame::try_from(i);
        if maybe_game.is_err() {
            continue;
        }
        let game = maybe_game.unwrap();
        let unpacked = game.get_game();
        let repacked:u32 = unpacked.clone().into();
        let reunpacked = PackedGame::try_from(repacked);
        if reunpacked.is_err() {
            println!("I: {:b}", i);
            println!("O: {:b}", repacked);
            dbg!(game.get_result());


            game.get_game().pretty_print();
            println!("{}", reunpacked.unwrap_err());
        }
        assert!(reunpacked.is_ok());
        if i != repacked || game != reunpacked.unwrap(){
            println!("I: {:b}", i);
            println!("O: {:b}", repacked);
            dbg!(game.get_result());
            dbg!(reunpacked.unwrap());
            dbg!(game);

        }
        assert_eq!(game, reunpacked.unwrap());
        assert_eq!(i, repacked);
    }
}














 