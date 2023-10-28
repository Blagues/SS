use std::collections::VecDeque;

const L: usize = 9;
const C: usize = 9;



struct State {
    board: [[u16; C]; L],
    // we save the board position as an u16 with the first bit being 1 if it is a filled in position.
    // And the next bits being the number that is filled in.
    // If we have an empty square (0 as first bit), we save the next 9 bits for the numbers that are
    // possible or not (1=yes) and the last 6 bits for the nr of possibilities.
}

impl State {
    fn find_min_options_position(&self) -> (usize, usize) {
        let mut min_value = u16::MAX; // Start with the maximum possible value
        let mut min_position = (0, 0);

        for l in 0..L {
            for c in 0..C {
                if self.is_filled_square(l, c) {continue};

                let length = self.board[l][c] & 63; // Extract last 6 bits
                if length < min_value {
                    min_value = length;
                    min_position = (l, c);
                }
            }
        }

        min_position
    }

    fn is_filled_square(&self, l: usize, c: usize) -> bool {
        let mask: u16 = 0b1000000000000000;
        // print!("{}", self.board[l][c] & !mask);
        // print!("F{}\n", (self.board[l][c] & mask) == mask);
        return (self.board[l][c] & mask) == mask;
    }


    fn get_options_for_cell(&self, l: usize, c: usize) -> Vec<usize> {
        // println!("A{}", l);
        // println!("B{}", c);
        if l >= L || c >= C || self.is_filled_square(l, c) {
            panic!("Out of bounds");
        }
        let pos_val = self.board[l][c];
        let mut options = Vec::new();

        for i in 1..=9 { // We skip 0 since it's the flag bit
            if (pos_val & (1 << (15 - i))) != 0 {
                options.push(i);
            }
        }
        options
    }
    fn set_board_val(&mut self, l: usize, c: usize, v: usize) {
        if v == 0 {
            // dont set flags of empty squares
            self.board[l][c] = 0;
            return;
        }
        self.board[l][c] = 0b1000000000000000 | v as u16;
    }

    fn get_board_val(&mut self, l: usize, c: usize) -> u16{
        if self.is_filled_square(l, c) {
            return self.board[l][c] & 0b0111111111111111;
        }
        else {
            return 0;
        }
    }

    fn is_full(&self) -> bool {
        for l in 0..L {
            for c in 0..C {
                if !self.is_filled_square(l, c) {
                    return false;
                }
            }
        }
        true
    }

    fn print(&mut self) -> bool {
        for l in 0..L {
            for c in 0..C {
                print!("{}", self.get_board_val(l ,c));
            }
            print!("\n");
        }
        print!("\n");
        true
    }

    fn update_pos(&mut self) {
        for l in 0..L {
            for c in 0..C {
                if !self.is_filled_square(l, c) { // Only update for empty cells
                    let mut options: u16 = 0b0111111111000000; // All options from 1 to 9 available initially (15th bit is unset as flag)

                    // Check row
                    for col in 0..C {
                        if self.is_filled_square(l, col) {
                            options &= !(1 << (15 - (self.board[l][col] & 0b0111111111111111) as usize));
                        }
                    }

                    // Check column
                    for lin in 0..L {
                        if self.is_filled_square(lin, c) {
                            options &= !(1 << (15 - (self.board[lin][c] & 0b0111111111111111) as usize));
                        }
                    }

                    // Check 3x3 grid
                    let grid_start_row = (l / 3) * 3;
                    let grid_start_col = (c / 3) * 3;
                    for row in grid_start_row..grid_start_row + 3 {
                        for col in grid_start_col..grid_start_col + 3 {
                            if self.is_filled_square(row, col) {
                                options &= !(1 << (15 - (self.board[row][col] & 0b0111111111111111) as usize));
                            }
                        }
                    }

                    // Calculate number of available options
                    let num_options = (options >> 6).count_ones() as u16;

                    // Clear the last 6 bits and then set them to represent the number of available options
                    options = (options & 0xFFC0) | num_options;

                    self.board[l][c] = options;
                }
            }
        }
    }


    fn clone(&self) -> Self {
        Self {
            board: self.board.clone(),
        }
    }
}

fn main() {
    let mut begin_state = State {
        board:[[0; C]; L],
    };

    let board: [[u16; C]; L] =  [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
    [6, 0, 0, 1, 9, 5, 0, 0, 0],
    [0, 9, 8, 0, 0, 0, 0, 6, 0],
    [8, 0, 0, 0, 6, 0, 0, 0, 3],
    [4, 0, 0, 8, 0, 3, 0, 0, 1],
    [7, 0, 0, 0, 2, 0, 0, 0, 6],
    [0, 6, 0, 0, 0, 0, 2, 8, 0],
    [0, 0, 0, 4, 1, 9, 0, 0, 5],
    [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    for l in 0..L {
        for c in 0..C {
            begin_state.set_board_val(l, c, board[l][c] as usize);
        }
    }

    begin_state.update_pos();

    let mut queue:VecDeque<State> = VecDeque::new();
    queue.push_back(begin_state);

    while !queue.is_empty() {
        let cur: State = queue.pop_front().unwrap();
        let (l, c) = cur.find_min_options_position();
        for v in cur.get_options_for_cell(l, c) {
            let mut new_state = cur.clone();

            new_state.set_board_val(l, c, v);

            if new_state.is_full() {
                new_state.print();
                break;
            }

            new_state.update_pos();
            queue.push_front(new_state);
        }
    }

}
