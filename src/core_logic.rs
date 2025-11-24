#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub board: [Cell; 9],
    pub current_turn: Player,
    pub winner: Option<Player>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [Cell::Empty; 9],
            current_turn: Player::X,
            winner: None,
        }
    }

    pub fn play_turn(&mut self, index: usize) -> Result<(), String> {
        if index >= 9 {
            return Err("Index out of bounds".to_string());
        }

        if self.winner.is_some() {
            return Err("Game is already over".to_string());
        }

        if let Cell::Occupied(_) = self.board[index] {
            return Err("Cell is already occupied".to_string());
        }

        self.board[index] = Cell::Occupied(self.current_turn);
        
        if let Some(winner) = self.check_winner() {
            self.winner = Some(winner);
        } else {
            self.current_turn = match self.current_turn {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        }

        Ok(())
    }

    pub fn check_winner(&self) -> Option<Player> {
        let winning_combinations = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
            [0, 4, 8], [2, 4, 6],            // Diagonals
        ];

        for combo in winning_combinations.iter() {
            let [a, b, c] = *combo;
            if let Cell::Occupied(player_a) = self.board[a] {
                if let Cell::Occupied(player_b) = self.board[b] {
                    if let Cell::Occupied(player_c) = self.board[c] {
                        if player_a == player_b && player_b == player_c {
                            return Some(player_a);
                        }
                    }
                }
            }
        }

        None
    }
    
    pub fn is_draw(&self) -> bool {
        self.winner.is_none() && self.board.iter().all(|cell| matches!(cell, Cell::Occupied(_)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let game = Game::new();
        assert_eq!(game.current_turn, Player::X);
        assert!(game.board.iter().all(|&c| c == Cell::Empty));
        assert_eq!(game.winner, None);
    }

    #[test]
    fn test_play_turn() {
        let mut game = Game::new();
        assert!(game.play_turn(0).is_ok());
        assert_eq!(game.board[0], Cell::Occupied(Player::X));
        assert_eq!(game.current_turn, Player::O);
        
        assert!(game.play_turn(0).is_err()); // Occupied
        assert!(game.play_turn(9).is_err()); // Out of bounds
    }

    #[test]
    fn test_row_win() {
        let mut game = Game::new();
        // X plays 0, 1, 2
        // O plays 3, 4
        game.play_turn(0).unwrap(); // X
        game.play_turn(3).unwrap(); // O
        game.play_turn(1).unwrap(); // X
        game.play_turn(4).unwrap(); // O
        game.play_turn(2).unwrap(); // X wins

        assert_eq!(game.winner, Some(Player::X));
    }

    #[test]
    fn test_diagonal_win() {
        let mut game = Game::new();
        // X plays 0, 4, 8
        game.play_turn(0).unwrap(); // X
        game.play_turn(1).unwrap(); // O
        game.play_turn(4).unwrap(); // X
        game.play_turn(2).unwrap(); // O
        game.play_turn(8).unwrap(); // X wins

        assert_eq!(game.winner, Some(Player::X));
    }
    
    #[test]
    fn test_draw() {
        let mut game = Game::new();
        // X O X
        // X O O
        // O X X
        let moves = [0, 1, 2, 4, 3, 5, 7, 6, 8];
        for m in moves {
            game.play_turn(m).unwrap();
        }
        assert!(game.is_draw());
        assert_eq!(game.winner, None);
    }
}
