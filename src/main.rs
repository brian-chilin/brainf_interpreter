use std::env;
use std::collections::VecDeque;


fn main() {
    let input: Vec<char> = env::args().collect::<Vec<String>>()[1].chars().collect();
    println!("{:?}", input);

    let mut cells: VecDeque<u64> = VecDeque::new();
    cells.push_back(0);
    let mut memory_pointer: usize = 0;
    let mut instruction_pointer: usize = 0;

    while instruction_pointer < input.len() {
        let i = input[instruction_pointer as usize];
        //println!("interpretting instruction {}", i);
        
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
                println!("{}  todo .", cells[memory_pointer] as u8);
            },
            '[' => {
                instruction_pointer += 1;
                println!("  todo [");
            },
            ']' => {
                instruction_pointer += 1;
                println!("  todo ]");
            },
            _ => instruction_pointer += 1
        }
    }
    println!("{:?}", (&cells,));
}
