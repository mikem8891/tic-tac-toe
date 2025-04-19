type Board = [char; 10];

fn main() {
    'game: loop {
        let difficulty = 'h';
        let mut board: Board = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut free_pos = get_free_positions(&board);
        println!("How many players (1 or 2 or 0 to quit):\n");
        let mut players = String::new();
        std::io::stdin().read_line(&mut players).expect("failed to get input");
        let players = match players.trim().parse::<u8>() {
            Ok(pos) => pos,
            Err(_) => {
                println!("\nError: not a number\n");
                continue;
            }
        };
        if players == 0 {
            println!("Bye!\n");
            break 'game;
        } else if !(players == 1 || players == 2) {
            println!("\nError: select 1 or 2 players\n");
            continue;
        }
        const TITLE: &str = "\n TIC-TAC-TOE \n";
        println!("{TITLE}\n{}", board_to_string(board));
        let mut turn = 'O';
        loop {
            println!("\"{turn}\" turn. Enter your position: \n");
            if turn == 'X' && players == 1 {
                let comp_pos = get_comp_pos(&board, difficulty);
                println!("{comp_pos}");
                board[comp_pos] = turn;
            } else {
                let mut turn_pos = String::new();
                std::io::stdin().read_line(&mut turn_pos).expect("failed to get input");
                let turn_pos = match turn_pos.trim().parse::<usize>() {
                    Ok(pos) => pos,
                    Err(_) => {
                        println!("\nError: not a number");
                        println!("{TITLE}\n{}", board_to_string(board));
                        continue;
                    }
                };
                if free_pos.iter().any(|&pos| pos == turn_pos) {
                    board[turn_pos] = turn;
                } else {
                    println!("\nError: position {turn_pos} is not a available");
                    println!("{TITLE}\n{}", board_to_string(board));
                    continue;
                }
            }
            println!("{TITLE}\n{}", board_to_string(board));

            let winner = get_winner(&board);
            if let Some(winner) = winner {
                println!("\"{}\" is the winner!\n", winner);
                break;
            }
            free_pos = get_free_positions(&board);
            if free_pos.is_empty() {
                println!("Game is a draw!\n");
                break;
            }

            turn = match turn {
                'O' => 'X',
                'X' => 'O',
                _   => unreachable!()
            }
        }
    }
}

fn board_to_string(board: Board) -> impl std::fmt::Display {
    let horintal_line = " ───┼───┼───\n";
    format!( "  {} │ {} │ {}\n", board[7], board[8], board[9]) + horintal_line +
    &format!("  {} │ {} │ {}\n", board[4], board[5], board[6]) + horintal_line +
    &format!("  {} │ {} │ {}\n", board[1], board[2], board[3])
}

fn get_free_positions(board: &Board) -> Vec<usize> {
    let mut free_pos = vec![];
    for i in 1..10 {
        if board[i] != 'X' && board[i] != 'O' {
            free_pos.push(i);
        } 
    }
    free_pos
}

fn get_winner(board: &Board) -> Option<char> {
    ['X', 'O'].iter().filter_map(|winner|
        IN_A_ROW_INDEX.iter()
            .any(|row| row.iter()
                .all(|pos| board[*pos] == *winner))
                .then_some(*winner)).next()
}

/// get the computer's position
fn get_comp_pos(board: &Board, difficulty: char) -> usize {
    let mut free_pos = get_free_positions(board);
    match difficulty {
        'e' => {
            let free_index: usize = fastrand::usize(..free_pos.len());
            free_pos[free_index]
        }
        'm' => {
            let mut blocks = vec![];
            for _tries in 0..(free_pos.len() - 1) {
                let index = fastrand::usize(..free_pos.len());
                let pos = free_pos.swap_remove(index);
                let mut scenario = board.clone();
                scenario[pos] = 'X';
                if get_winner(&scenario).is_some() {
                    return pos;
                }
                scenario[pos] = 'O';
                if get_winner(&scenario).is_some() {
                    blocks.push(pos);
                }
            }
            if !blocks.is_empty() {
                let rand_index = fastrand::usize(..blocks.len());
                blocks[rand_index]
            } else {
                let free_pos = get_free_positions(board);
                let free_index: usize = fastrand::usize(..free_pos.len());
                free_pos[free_index]
            }
        }
        'h' => {
            let mut blocks = vec![];
            for &pos in &free_pos {
                let mut scenario = board.clone();
                scenario[pos] = 'X';
                if get_winner(&scenario).is_some() {
                    return pos;
                }
                scenario[pos] = 'O';
                if get_winner(&scenario).is_some() {
                    blocks.push(pos);
                }
            }
            if !blocks.is_empty() {
                let rand_index = fastrand::usize(..blocks.len());
                blocks[rand_index]
            } else {
                let free_index: usize = fastrand::usize(..free_pos.len());
                free_pos[free_index]
            }
        }
        _ => unreachable!()
    }
}

const IN_A_ROW_INDEX: [[usize; 3];8] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
    [1, 4, 7],
    [2, 5, 8],
    [3, 6, 9],
    [1, 5, 9],
    [3, 5, 7]
];