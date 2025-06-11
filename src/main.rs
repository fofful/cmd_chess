mod board;
use board::Board;
mod pawn;
use pawn::Pawn;
use std::io;
use regex::Regex;

fn main() {
    let mut board = Board::new(String::from("blue"));
    let mut pawns = Vec::new();
    let mut pawn01 = Pawn::new((0, 0), String::from("red"), String::from("rr"), String::from("\x1B[31m"), true);
    let mut pawn02 = Pawn::new((0, 1), String::from("red"), String::from("rh"), String::from("\x1B[31m"), true);
    let mut pawn03 = Pawn::new((0, 2), String::from("red"), String::from("rb"), String::from("\x1B[31m"), true);
    let mut pawn04 = Pawn::new((0, 3), String::from("red"), String::from("rk"), String::from("\x1B[31m"), true);
    let mut pawn05 = Pawn::new((0, 4), String::from("red"), String::from("rq"), String::from("\x1B[31m"), true);
    let mut pawn06 = Pawn::new((0, 5), String::from("red"), String::from("rn"), String::from("\x1B[31m"), true);
    let mut pawn07 = Pawn::new((0, 6), String::from("red"), String::from("rh"), String::from("\x1B[31m"), true);
    let mut pawn08 = Pawn::new((0, 7), String::from("red"), String::from("rr"), String::from("\x1B[31m"), true);
    let mut pawn09 = Pawn::new((1, 0), String::from("red"), String::from("rp"), String::from("\x1B[31m"), true);
    let mut pawn10 = Pawn::new((1, 1), String::from("red"), String::from("rp"), String::from("\x1B[31m"), true);
    let mut pawn11 = Pawn::new((1, 2), String::from("red"), String::from("rp"), String::from("\x1B[31m"), true);
    let mut pawn12 = Pawn::new((1, 3), String::from("red"), String::from("rp"), String::from("\x1B[31m"), true);
    let mut pawn13 = Pawn::new((1, 4), String::from("red"), String::from("rp"), String::from("\x1B[31m"), true);
    let mut pawn14 = Pawn::new((1, 5), String::from("red"), String::from("rp"), String::from("\x1B[31m"), true);
    let mut pawn15 = Pawn::new((1, 6), String::from("red"), String::from("rp"), String::from("\x1B[31m"), true);
    let mut pawn16 = Pawn::new((1, 7), String::from("red"), String::from("rp"), String::from("\x1B[31m"), true);
    let mut pawn17 = Pawn::new((7, 0), String::from("blue"), String::from("br"), String::from("\x1B[34m"), true);
    let mut pawn18 = Pawn::new((7, 1), String::from("blue"), String::from("bh"), String::from("\x1B[34m"), true);
    let mut pawn19 = Pawn::new((7, 2), String::from("blue"), String::from("bb"), String::from("\x1B[34m"), true);
    let mut pawn20 = Pawn::new((7, 3), String::from("blue"), String::from("bk"), String::from("\x1B[34m"), true);
    let mut pawn21 = Pawn::new((7, 4), String::from("blue"), String::from("bq"), String::from("\x1B[34m"), true);
    let mut pawn22 = Pawn::new((7, 5), String::from("blue"), String::from("bn"), String::from("\x1B[34m"), true);
    let mut pawn23 = Pawn::new((7, 6), String::from("blue"), String::from("bh"), String::from("\x1B[34m"), true);
    let mut pawn24 = Pawn::new((7, 7), String::from("blue"), String::from("br"), String::from("\x1B[34m"), true);
    let mut pawn25 = Pawn::new((6, 0), String::from("blue"), String::from("bp"), String::from("\x1B[34m"), true);
    let mut pawn26 = Pawn::new((6, 1), String::from("blue"), String::from("bp"), String::from("\x1B[34m"), true);
    let mut pawn27 = Pawn::new((6, 2), String::from("blue"), String::from("bp"), String::from("\x1B[34m"), true);
    let mut pawn28 = Pawn::new((6, 3), String::from("blue"), String::from("bp"), String::from("\x1B[34m"), true);
    let mut pawn29 = Pawn::new((6, 4), String::from("blue"), String::from("bp"), String::from("\x1B[34m"), true);
    let mut pawn30 = Pawn::new((6, 5), String::from("blue"), String::from("bp"), String::from("\x1B[34m"), true);
    let mut pawn31 = Pawn::new((6, 6), String::from("blue"), String::from("bp"), String::from("\x1B[34m"), true);
    let mut pawn32 = Pawn::new((6, 7), String::from("blue"), String::from("bp"), String::from("\x1B[34m"), true);
    pawns.push(&mut pawn01);
    pawns.push(&mut pawn02);
    pawns.push(&mut pawn03);
    pawns.push(&mut pawn04);
    pawns.push(&mut pawn05);
    pawns.push(&mut pawn06);
    pawns.push(&mut pawn07);
    pawns.push(&mut pawn08);
    pawns.push(&mut pawn09);
    pawns.push(&mut pawn10);
    pawns.push(&mut pawn11);
    pawns.push(&mut pawn12);
    pawns.push(&mut pawn13);
    pawns.push(&mut pawn14);
    pawns.push(&mut pawn15);
    pawns.push(&mut pawn16);
    pawns.push(&mut pawn17);
    pawns.push(&mut pawn18);
    pawns.push(&mut pawn19);
    pawns.push(&mut pawn20);
    pawns.push(&mut pawn21);
    pawns.push(&mut pawn22);
    pawns.push(&mut pawn23);
    pawns.push(&mut pawn24);
    pawns.push(&mut pawn25);
    pawns.push(&mut pawn26);
    pawns.push(&mut pawn27);
    pawns.push(&mut pawn28);
    pawns.push(&mut pawn29);
    pawns.push(&mut pawn30);
    pawns.push(&mut pawn31);
    pawns.push(&mut pawn32);
    board.draw_board(&pawns);
    pawns[0].set_alive(false);
    board.draw_board(&pawns);
    

    //game loop
    while board.get_game_ended() == false{
        println!("{} turn", board.get_player_turn());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        println!("entry: {}", input.trim());

        //input validation
        let re = Regex::new(r"[A-H][1-8] [A-H][1-8]").unwrap();
        if !re.is_match(input.as_str()){
            println!("Wrong input format.");
            continue
        }

        //input transformation
        let mut char_vec: Vec<char> = input.chars().collect();
        let transformed_vec: Vec<usize> = char_vec
            .iter_mut()
            .map(|c|{
            if c.is_alphabetic(){
                (*c as u8 - b'A') as usize
            } else if c.is_whitespace(){
                *c as usize
            } 
            else {
                (*c as u8 - 49) as usize
            }
        }).collect();
        let move_from = (transformed_vec[0], transformed_vec[1]);
        let move_to = (transformed_vec[3], transformed_vec[4]);

        println!("this is the move_from: {:#?}", move_from);
        println!("this is the move_to: {:#?}", move_to);

        //validate move_from position
        if !pawns.iter()
                .any(|p| move_from == *p.get_position() && board.get_player_turn() == p.get_player()){
                    println!("Invalid move_from position");
                    continue
                };

        //validate move_to position (need to rethink)
        if !pawns.iter()
                .any(|p| move_to == *p.get_position() && board.get_player_turn() != p.get_player()){
                    println!("Invalid move_to position");
                    continue
                };
        
        if let Some(found_pawn) = pawns.iter_mut().find(|p| *p.get_position() == move_to){
            found_pawn.set_alive(false);
        };
        if let Some(found_pawn) = pawns.iter_mut().find(|p| *p.get_position() == move_from){
            found_pawn.set_position(move_to);
        };
        
        pawns.iter().for_each(|p| println!("{:#?}", p));
        board.draw_board(&pawns);

        //switch turn
        if board.get_player_turn() == "blue"{
            board.set_player_turn(String::from("red"));
        } else {
            board.set_player_turn(String::from("blue"));
        }
    }

    println!("Game has ended.");
}