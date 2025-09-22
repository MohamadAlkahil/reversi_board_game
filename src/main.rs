use std::collections::HashMap;
use std::io;
/*
Breif Explanation: Runs the game and declares the winner.

Parameters: None

Returns: None

Conceptual Flow:
    1.intializes the reversi board, sets the intial turn to Black, and says both players have valid moves
    2.loop
        a.if both players dont have any valid moves break out of step 2.
        b.get all the valid moves for the current player
        c.if the current player has no valid moves then switch turns and go to step 2 a.
          otherwise get a valid input for (RowCol) and if it is within the valid moves update the board. reset the variable keeping track how many times no valid moves have been reached.
    3.print board and declare winner
*/
fn main() {
    let mut board = [['.'; 8]; 8];
    board[3][3] = 'W';
    board[4][3] = 'B';
    board[3][4] = 'B';
    board[4][4] = 'W';

    let mut turn = 'B';
    let mut no_moves = 0;

    loop {
        if no_moves > 1 {
            break;
        }
        let valid_moves: HashMap<(usize, usize), Vec<(i32, i32)>> = get_moves(turn, board);
        if !valid_moves.is_empty() {
            loop {
                print_board(board);
                if no_moves==1{
                    if turn=='B'{
                        println!("W player has no valid move.");
                    }
                    else{
                        println!("B player has no valid move.");
                    }
                }
                let (position_r, position_c): (usize, usize) = string_to_position(turn).into();
                if !valid_moves.contains_key(&(position_r, position_c)) {
                    println!("Invalid move. Try again.");
                    continue;
                }
                board = update_board(turn, (position_r, position_c), board, valid_moves);
                break;
            }
            no_moves = 0;
        } else {
            no_moves += 1;
        }
        turn = if turn == 'B' { 'W' } else { 'B' };
    }
    print_board(board);
    if turn=='B'{
        println!("B player has no valid move.");
        println!("W player has no valid move.");
    }
    else{
        println!("W player has no valid move.");
        println!("B player has no valid move.");
    }
    determine_winner(board);
}

/*
Breif Explanation: Goes through the reversi board and prints if there is a draw or Black or White won.

Parameters: 
    board: [[char; 8]; 8] - the reversi board

Returns: None
*/
fn determine_winner(board: [[char; 8]; 8]){
    let mut b_count: usize =0;
    let mut w_count: usize=0;
    for r in 0..8 {
        for c in 0..8 {
            if board[r][c] == 'B' {
                b_count+=1;
            }
            else if board[r][c] == 'W' {
                w_count+=1;
            }
        }
    }
    if b_count>w_count{
        let diff=b_count-w_count;
        println!("Black wins by {diff} points!");
    }
    else if b_count<w_count{
        let diff=w_count-b_count;
        println!("White wins by {diff} points!");
    }
    else{
        println!("Draw!");
    }

}

/*
Breif Explanation: Iterates all valid moves for a specified player at a given position and updates the reversi board in all valid directions associated with the given position.

Parameters: 
    turn: char - the current player
    (position_r,position_c): (usize,usize) - the position on the reversi board where a piece is being placed
    board: [[char; 8]; 8] - the mutable reversi board
    all_valid_moves: HashMap<(usize, usize), Vec<(i32, i32)>> - the valid moves for the current player

Returns:
    board: [[char; 8]; 8] - the updated reversi board
*/
fn update_board(
    turn: char,
    (position_r, position_c): (usize, usize),
    mut board: [[char; 8]; 8],
    all_valid_moves: HashMap<(usize, usize), Vec<(i32, i32)>>,
) -> [[char; 8]; 8] {
    let moves = all_valid_moves.get(&(position_r, position_c)).unwrap();
    board[position_r][position_c] = turn;
    for (dr, dc) in moves.iter() {
        let mut nr = (position_r as i32) + dr;
        let mut nc = (position_c as i32) + dc;
        while 0 <= nr && nr < 8 && 0 <= nc && nc < 8 && board[nr as usize][nc as usize] != turn {
            board[nr as usize][nc as usize] = turn;
            nr = nr + dr;
            nc = nc + dc;
        }
    }
    return board;
}

/*
Breif Explanation: Takes in a move for the current player and determines if it is valid and returning the associated position on the board  

Parameters: 
    turn: char - the current player
    
Returns:
    positions: [usize; 2] - the position on the reversi board associated with the IO value
*/
fn string_to_position(turn: char) -> [usize; 2] {
    let char_to_int: HashMap<char, usize> = HashMap::from([
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
    ]);
    let mut positions: [usize; 2] = [0; 2];
    'outter: loop {
        print!("Enter move for colour {turn} (RowCol): ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let len_guess = guess.trim().chars().count();
        if len_guess != 2 {
            println!("Invalid move. Try again.");
            positions = [0, 0];
            continue 'outter;
        }
        for (i, e) in guess.trim().chars().enumerate() {
            let check: bool = match e {
                'a'..='h' => true,
                _ => false,
            };
            if !check {
                positions = [0, 0];
                println!("Invalid move. Try again.");
                continue 'outter;
            }
            positions[i] = char_to_int[&e];
        }
        return positions;
    }
}

/*
Breif Explanation: prints the reversi board  

Parameters: 
    board: [[char; 8]; 8] - the reversi board
    
Returns: None
*/
fn print_board(board: [[char; 8]; 8]) {
    let int_to_char: HashMap<usize, char> = HashMap::from([
        (0, 'a'),
        (1, 'b'),
        (2, 'c'),
        (3, 'd'),
        (4, 'e'),
        (5, 'f'),
        (6, 'g'),
        (7, 'h'),
    ]);
    println!("  abcdefgh");
    for r in 0usize..8usize {
        print!("{} ", int_to_char[&r]);
        for c in 0usize..8usize {
            print!("{}", board[r][c]);
        }
        println!("");
    }
}

/*
Breif Explanation: Scans the entire reversi board for valid moves for the current player.

Parameters: 
    turn: char - the current player
    board: [[char; 8]; 8] - the mutable reversi board
    
Returns:
    valid_moves: HashMap<(usize, usize), Vec<(i32, i32)>> - the valid moves for the current player

Conceptual Flow:
    1.intialize hashmap where a key that is a board position has a value a vector that holds the direction of enemy piece(s) that can be sandwhiched by firendly pieces. This hashmap will represent the valid moves for the current player.
    2.iterate through every position on the board:
        a.if an empty position is reached check all 8 directions:
            1.if an enemy piece in a direction is found continue heading in that direction to see if a friendly piece can be reached to create a sandwhich. If a sandwhich can be created then valid_moves[position].push(direction). Continue to check the rest of the directions to see if any other direction creates a sandwhich.
    3.return the valid moves for the current player   
*/
fn get_moves(turn: char, board: [[char; 8]; 8]) -> HashMap<(usize, usize), Vec<(i32, i32)>> {
    let mut valid_moves: HashMap<(usize, usize), Vec<(i32, i32)>> = HashMap::new();
    for r in 0..8 {
        for c in 0..8 {
            if board[r][c] == '.' {
                let directions: [(i32, i32); 8] = [
                    (0, 1),
                    (1, 0),
                    (0, -1),
                    (-1, 0),
                    (1, 1),
                    (-1, 1),
                    (1, -1),
                    (-1, -1),
                ];
                for (dr, dc) in directions {
                    let mut nr = (r as i32) + dr;
                    let mut nc = (c as i32) + dc;
                    if 0 <= nr
                        && nr < 8
                        && 0 <= nc
                        && nc < 8
                        && board[nr as usize][nc as usize] != '.'
                        && board[nr as usize][nc as usize] != turn
                    {
                        while 0 <= nr
                            && nr < 8
                            && 0 <= nc
                            && nc < 8
                            && board[nr as usize][nc as usize] != '.'
                            && board[nr as usize][nc as usize] != turn
                        {
                            nr = nr + dr;
                            nc = nc + dc;
                        }
                        if 0 <= nr
                            && nr < 8
                            && 0 <= nc
                            && nc < 8
                            && board[nr as usize][nc as usize] != '.'
                            && board[nr as usize][nc as usize] == turn
                        {
                            valid_moves
                                .entry((r, c))
                                .or_insert_with(Vec::new)
                                .push((dr, dc));
                        }
                    }
                }
            }
        }
    }
    return valid_moves;
}
