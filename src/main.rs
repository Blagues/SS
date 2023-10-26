use std::collections::VecDeque;

const L: usize = 9;
const C: usize = 9;

struct State {
    board: [[u8; C]; L],
    pos: [[u16; C]; L],
}

impl State {
    fn find_min_options_position(&self) -> (usize, usize) {
        let mut min_value = u16::MAX; // Start with the maximum possible value
        let mut min_position = (0, 0);

        for l in 0..L {
            for c in 0..C {
                let length = self.pos[l][c] & 63; // Extract last 6 bits
                if length < min_value && length > 0 {
                    min_value = length;
                    min_position = (l, c);
                }
            }
        }

        min_position
    }

    fn get_options_for_cell(&self, l: usize, c: usize) -> Vec<u8> {
        if l >= L || c >= C {
            panic!("Out of bounds");
        }
        let pos_val = self.pos[l][c];
        let mut options = Vec::new();
        for i in 1..=9 { // We skip 0 since it's never an option
            if (pos_val & (1 << (15 - i))) != 0 {
                options.push(i);
            }
        }
        options
    }

    // Removes a specific option and sets the opt array to all zeros
    fn clear_opt_at(&mut self, l: usize, c: usize) {
        if c < C && l < L {
            self.pos[l][c] = 0;
        } else {
            panic!("Out of bounds or invalid index");
        }
    }

    fn set_board(&mut self, l: usize, c: usize, v: u8) {
        if c < C && l < L {
            self.board[l][c] = v;
            self.clear_opt_at(l, c);
        } else {
            panic!("Out of bounds or invalid index");
        }
    }

    fn is_full(&self) -> bool {
        for l in 0..L {
            for c in 0..C {
                if self.board[l][c] == 0 {
                    return false;
                }
            }
        }
        true
    }

    fn print(&self) -> bool {
        for l in 0..L {
            for c in 0..C {
                print!("{}", self.board[l][c]);
            }
            print!("\n");
        }
        print!("\n");
        true
    }

    // fn remove_option_at(&mut self, c: usize, l: usize, opt: usize) {
    //     if c < C && l < L && opt < 10 {
    //         let adjusted_idx = 15 - opt; // Adjust the index
    //
    //         // Set the specific bit to 0
    //         self.pos[l][c] &= !(1 << adjusted_idx);
    //
    //         // Decrease the number of options by 1
    //         let options_count = self.pos[l][c] & 63; // Get current count
    //         self.pos[l][c] = (self.pos[l][c] & !63) | (options_count - 1);
    //     } else {
    //         panic!("Out of bounds or invalid index");
    //     }
    // }

    fn update_pos(&mut self) {
        for l in 0..L {
            for c in 0..C {
                if self.board[l][c] == 0 { // Only update for empty cells
                    let mut options: u16 = 0b0111111111000000; // All options from 1 to 9 available initially (15th bit is unset for option 0)

                    // Check row
                    for col in 0..C {
                        if self.board[l][col] != 0 {
                            options &= !(1 << (15 - self.board[l][col] as usize));
                        }
                    }

                    // Check column
                    for row in 0..L {
                        if self.board[row][c] != 0 {
                            options &= !(1 << (15 - self.board[row][c] as usize));
                        }
                    }

                    // Check 3x3 grid
                    let grid_start_row = (l / 3) * 3;
                    let grid_start_col = (c / 3) * 3;
                    for row in grid_start_row..grid_start_row + 3 {
                        for col in grid_start_col..grid_start_col + 3 {
                            if self.board[row][col] != 0 {
                                options &= !(1 << (15 - self.board[row][col] as usize));
                            }
                        }
                    }

                    // Calculate number of available options
                    let num_options = (options >> 6).count_ones() as u16;

                    // Clear the last 6 bits and then set them to represent the number of available options
                    options = (options & 0xFFC0) | num_options;

                    self.pos[l][c] = options;
                }
            }
        }
    }


    fn clone(&self) -> Self {
        Self {
            board: self.board.clone(),
            pos: self.pos.clone(),
        }
    }
}

fn main() {
    let mut begin_state = State {
        board: [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ],
        pos: [[0; C]; L],
    };

    begin_state.update_pos();

    let mut queue:VecDeque<State> = VecDeque::new();
    queue.push_back(begin_state);

    while !queue.is_empty() {
        let cur: State = queue.pop_front().unwrap();
        let (l, c) = cur.find_min_options_position();
        for v in cur.get_options_for_cell(l, c) {
            let mut new_state = cur.clone();

            new_state.set_board(l, c, v);

            if new_state.is_full() {
                new_state.print();
                break;
            }

            new_state.update_pos();
            queue.push_front(new_state);
        }
    }

}
