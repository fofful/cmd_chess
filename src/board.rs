<<<<<<< HEAD
use super::pawn::Pawn;

pub struct Board{
    player_turn: String,
    game_ended: bool,
    table: [[String; 8]; 8],
}

impl Board{
    pub fn new(player_turn: String) -> Self{
        let table: [[String; 8]; 8] = [[String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")]];
        Board{player_turn, game_ended: false, table}
    }

    pub fn set_player_turn(&mut self, player: String){
        self.player_turn = player;
    }

    pub fn get_player_turn(&self) -> &str{
        self.player_turn.as_str()
    }

    pub fn set_game_ended(&mut self, game: bool){
        self.game_ended = game
    }

    pub fn get_game_ended(&self) -> bool{
        self.game_ended
    }

    pub fn draw_board(&mut self, pawns: &Vec<&mut Pawn>){
        let empty = String::from("  ");
        for row in &mut self.table{
            row.iter_mut()
                    .for_each(|pos| *pos = empty.clone());
        }
        
        for pawn in pawns{
            let position = pawn.get_position();
            let class = pawn.get_class();
            let color = pawn.get_color();
            let is_live = pawn.get_alive();
            if is_live{
                self.table[position.0][position.1] = format!("{}{}{}", color.to_string(), class.to_string(), "\x1B[38;5;130m");
            }
        }

        println!("\x1B[38;5;130m.---------------------------------------.\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[0][0], self.table[0][1], self.table[0][2], self.table[0][3], self.table[0][4], self.table[0][5], self.table[0][6], self.table[0][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[1][0], self.table[1][1], self.table[1][2], self.table[1][3], self.table[1][4], self.table[1][5], self.table[1][6], self.table[1][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[2][0], self.table[2][1], self.table[2][2], self.table[2][3], self.table[2][4], self.table[2][5], self.table[2][6], self.table[2][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[3][0], self.table[3][1], self.table[3][2], self.table[3][3], self.table[3][4], self.table[3][5], self.table[3][6], self.table[3][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[4][0], self.table[4][1], self.table[4][2], self.table[4][3], self.table[4][4], self.table[4][5], self.table[4][6], self.table[4][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[5][0], self.table[5][1], self.table[5][2], self.table[5][3], self.table[5][4], self.table[5][5], self.table[5][6], self.table[5][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[6][0], self.table[6][1], self.table[6][2], self.table[6][3], self.table[6][4], self.table[6][5], self.table[6][6], self.table[6][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[7][0], self.table[7][1], self.table[7][2], self.table[7][3], self.table[7][4], self.table[7][5], self.table[7][6], self.table[7][7]);
        println!("\x1B[38;5;130m'---------------------------------------'\x1B[0m");
    }
=======
use super::pawn::Pawn;

pub struct Board{
    player_turn: String,
    game_ended: bool,
    table: [[String; 8]; 8],
}

impl Board{
    pub fn new(player_turn: String) -> Self{
        let table: [[String; 8]; 8] = [[String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")],
                                            [String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  "), String::from("  ")]];
        Board{player_turn, game_ended: false, table}
    }

    pub fn set_player_turn(&mut self, player: String){
        self.player_turn = player;
    }

    pub fn get_player_turn(&self) -> &str{
        self.player_turn.as_str()
    }

    pub fn set_game_ended(&mut self, game: bool){
        self.game_ended = game
    }

    pub fn get_game_ended(&self) -> bool{
        self.game_ended
    }

    pub fn draw_board(&mut self, pawns: &Vec<&mut Pawn>){
        let empty = String::from("  ");
        for row in &mut self.table{
            row.iter_mut()
                    .for_each(|pos| *pos = empty.clone());
        }
        
        for pawn in pawns{
            let position = pawn.get_position();
            let class = pawn.get_class();
            let color = pawn.get_color();
            let is_live = pawn.get_alive();
            if is_live{
                self.table[position.0][position.1] = format!("{}{}{}", color.to_string(), class.to_string(), "\x1B[38;5;130m");
            }
        }

        println!("\x1B[38;5;130m.---------------------------------------.\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[0][0], self.table[0][1], self.table[0][2], self.table[0][3], self.table[0][4], self.table[0][5], self.table[0][6], self.table[0][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[1][0], self.table[1][1], self.table[1][2], self.table[1][3], self.table[1][4], self.table[1][5], self.table[1][6], self.table[1][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[2][0], self.table[2][1], self.table[2][2], self.table[2][3], self.table[2][4], self.table[2][5], self.table[2][6], self.table[2][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[3][0], self.table[3][1], self.table[3][2], self.table[3][3], self.table[3][4], self.table[3][5], self.table[3][6], self.table[3][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[4][0], self.table[4][1], self.table[4][2], self.table[4][3], self.table[4][4], self.table[4][5], self.table[4][6], self.table[4][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[5][0], self.table[5][1], self.table[5][2], self.table[5][3], self.table[5][4], self.table[5][5], self.table[5][6], self.table[5][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[6][0], self.table[6][1], self.table[6][2], self.table[6][3], self.table[6][4], self.table[6][5], self.table[6][6], self.table[6][7]);
        println!("\x1B[38;5;130m|----|----|----|----|----|----|----|----|\x1B[0m");
        println!("\x1B[38;5;130m| {} | {} | {} | {} | {} | {} | {} | {} |\x1B[0m", self.table[7][0], self.table[7][1], self.table[7][2], self.table[7][3], self.table[7][4], self.table[7][5], self.table[7][6], self.table[7][7]);
        println!("\x1B[38;5;130m'---------------------------------------'\x1B[0m");
    }
>>>>>>> db6b194 (initial)
}