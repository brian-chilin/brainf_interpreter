use std::env;
use std::collections::VecDeque;


fn main() {
    let input: Vec<char> = env::args().collect::<Vec<String>>()[1].chars().collect();
    //println!("{:?}", input);

    let mut cells: VecDeque<u8> = VecDeque::new();
    cells.push_back(0);
    let mut memory_pointer: usize = 0;
    let mut instruction_pointer: usize = 0;

    while instruction_pointer < input.len() {
        let i = input[instruction_pointer];
        //println!("interpreting instruction {}", i);
        match i {
            '>' => {
                memory_pointer += 1;
                if memory_pointer == cells.len() {
                    cells.push_back(0);
                }
                instruction_pointer += 1;
            },
            '<' => {
                if memory_pointer == 0 {
                    cells.push_front(0);
                } else {
                    memory_pointer -= 1;
                }
                instruction_pointer += 1;
            },
            '+' => {
                cells[memory_pointer] += 1;
                instruction_pointer += 1;
            },
            '-' => {
                cells[memory_pointer] -= 1;
                instruction_pointer += 1;
            },
            '.' => {
                instruction_pointer += 1;
                print!("{}", cells[memory_pointer] as char);
            },
            '[' => {
                if cells[memory_pointer] == 0 {
                    while input[instruction_pointer] != ']' {
                        instruction_pointer += 1
                    }
                    instruction_pointer += 1
                } else {
                    let mut bcount: usize = 1;
                    while bcount > 0 {
                        instruction_pointer += 1;
                        match input[instruction_pointer] {
                            '[' => bcount += 1,
                            ']' => bcount -= 1,
                            _k => () // do nothing
                        }
                    }
                }
            },
            ']' => {
                if cells[memory_pointer] != 0 {
                    let mut bcount: usize = 1;
                    while bcount > 0 {
                        instruction_pointer -= 1;
                        match input[instruction_pointer] {
                            '[' => bcount -= 1,
                            ']' => bcount += 1,
                            _k => () // do nothing
                        }
                    }
                    instruction_pointer += 1;
                } else {
                    instruction_pointer += 1;
                }
            },
            _c => {
                println!("Continuing passed unknown instruction {}", _c);
                instruction_pointer += 1
            }
        }
    }
    //println!("{:?}", (&cells));
}
