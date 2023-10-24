use std::cmp::min;

const L:usize = 9;
const C:usize = L;

type Board = [[u8; C]; L];
type PosArray = [bool; 10];

fn main() {
    let puzzle = sudokugen::Puzzle::generate(sudokugen::BoardSize::NineByNine);
    let board_generated = puzzle.board();
    let _ans = puzzle.solution();

    let mut board: [[u8; C]; L] = [[0; C]; L]; // TEST using other types like u32
    let mut pos_nrs: [[[bool; 10]; C]; L] = [[[false; 10]; C]; L];

    // set board
    for l in 0..L {
        for c in 0..C {
            board[l][c] = board_generated.get_at(l, c).unwrap_or(0);
        }
    }

    let board_is_full:bool =  'outer: {
         for l in 0..L {
            for c in 0..C {
                if board[l][c] ==  0 { break 'outer false; }
            }
        }
        true
    };

    let get_available_numbers = |l: usize, c: usize, board: &Board| {
        let nr: u8 = board[l][c];
        let mut nrs_available: PosArray = [true; 10];

        for i in 0..C {
            if board[l][i] == nr {
                nrs_available[board[l][i] as usize] = false;
            }
        }

        for i in 0..L {
            if board[i][c] == nr {
                nrs_available[board[l][i] as usize] = false;
            }
        }

        let gsl:u8 = (l / 3) as u8 * 3;
        let gsc:u8 = (l / 3) as u8 * 3;

        for i in 0..3 {
            for j in 0..3 {
                let line = min((gsl + i) as usize,  L - 1) ;
                let col = min((gsc + j) as usize,  L - 1) ;
                if board[line][col] == nr {
                    nrs_available[board[line][col] as usize] = false;
                }
            }
        }

        return nrs_available;
    };

    fn print_board(board: Board) {
        for l in 0..L {
            for c in 0..C {
                print!("{}", board[l][c]);
            }
            print!("\n");
        }
    }

    let do_first_single_guess = 'outer: {
        for l in 0..L {
            for c in 0..C {
                if pos_nrs[l][c].iter().map(|&val| if val { 1u8 } else { 0u8 }).sum::<u8>() == 1 {
                    for i in 0..10 {
                        if pos_nrs[l][c][i] == true {board[l][c] = i as u8; break 'outer true}
                    }
                }
            }
        }
        false
    };

    let mut i = 0;

    while !board_is_full {
        // find all possible guesses
        for l in 0..L {
            for c in 0..C {
                if board[l][c] == 0 {
                    pos_nrs[l][c] = get_available_numbers(l, c, &board);
                }
            }
        }

        println!("T {}", i);
        print_board(board);
        i += 1;

        // go again if we found square with only 1 option
        if do_first_single_guess { continue }

        panic!("no easy");

        // else we have to make a guess

    }





}



