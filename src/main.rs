use std::io::stdin;
use std::process;

/// print the board
/// ### Arguments
/// * `board` - a vector representation of game board
#[rustfmt::skip]
fn print_board(board: &[u32]) {
    for i in (0..9).step_by(3) {
        let first = if board[i] == 0 { "-" } else {
            if board[i] == 1 { "X" } else { "O" }
        };
        let second = if board[i + 1] == 0 { "-" } else {
            if board[i + 1] == 1 { "X" } else { "O" }
        };
        let third = if board[i + 2] == 0 { "-" } else {
            if board[i + 2] == 1 { "X" } else { "O" }
        };
        println!("{} | {} | {}", first, second, third);
    }
    println!("{:?}", board);
}

/// return user input
fn take_input() -> u32 {
    let mut line = String::new();
    println!("Enter your move (1 - 9) :");
    stdin().read_line(&mut line).unwrap();
    line.trim().parse::<u32>().unwrap_or(10) - 1
}

/// perform a move on the board
/// ### Arguments:
/// `board` - a vector representation of game board
/// `move_place` - where the next move has to be performed
/// `player` - whose move has to performed
fn make_move(board: &mut [u32], move_place: u32, player: u32) {
    if move_place > 8 || board[move_place as usize] != 0 {
        println!("wrong move input provided");
        let new_input: u32 = take_input();
        make_move(board, new_input, player)
    }

    board[move_place as usize] = player;
}

/// perform a move on the board
/// ### Arguments:
/// `board` - a vector representation of game board
/// `combos` - vector of arrays of winning combinations
fn check_game(board: &[u32], combos: &Vec<[u32; 3]>) -> u32 {
    for combo in combos {
        if board[combo[0] as usize] == board[combo[1] as usize]
            && board[combo[1] as usize] == board[combo[2] as usize]
            && board[combo[1] as usize] != 0
        {
            return board[combo[0] as usize];
        }
    }
    0
}

/// return changed player
/// ### Arguments:
/// `player` - current player
fn change_player(player: u32) -> u32 {
    if player == 1 {
        2
    } else {
        1
    }
}

fn main() {
    let mut board: Vec<u32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

    let win_combos = vec![
        // horizontal lines
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        // vertical lines
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        // cross lines
        [0, 4, 8],
        [2, 4, 6],
    ];

    let mut player = 1;
    let mut current_input: u32;
    let mut is_running = true;
    let mut moves_made: u32 = 0;

    // main game loop
    while is_running && moves_made < 9 {
        print_board(&board);
        println!("current player is: {}", player);

        current_input = take_input();
        make_move(&mut board, current_input, player);
        moves_made += 1;

        let game_status = check_game(&board, &win_combos);

        if game_status == 1 || game_status == 2 {
            is_running = false;
            println!(
                "{} is the Winner!",
                if game_status == 1 { "X" } else { "O" }
            );
        }
        player = change_player(player);
    }

    if moves_made == 9 {
        println!("that's a Draw!");
    }

    process::exit(0);
}
