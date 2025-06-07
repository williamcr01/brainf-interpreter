use std::io::Read;

#[derive(Clone)]
enum Instructions {
    IncrementPtr,
    DecrementPtr,
    IncrementByte,
    DecrementByte,
    OutputByte,
    InputByte,
    BeginLoop,
    EndLoop,
}

fn parse_instructions(input: &str) -> Result<Vec<Instructions>, String> {
    let mut instructions = Vec::new();
    let mut loop_counter = 0;

    for c in input.chars() {
        match c {
            '>' => instructions.push(Instructions::IncrementPtr),
            '<' => instructions.push(Instructions::DecrementPtr),
            '+' => instructions.push(Instructions::IncrementByte),
            '-' => instructions.push(Instructions::DecrementByte),
            '.' => instructions.push(Instructions::OutputByte),
            ',' => instructions.push(Instructions::InputByte),
            '[' => {
                instructions.push(Instructions::BeginLoop);
                loop_counter += 1;
            }
            ']' => {
                if loop_counter == 0 {
                    return Err("Loop error: ']' without a matching '['.".to_string());
                }
                instructions.push(Instructions::EndLoop);
                loop_counter -= 1;
            }
            _ => { /* Ignore other characters */ }
        }
    }

    if loop_counter != 0 {
        Err("Loop error: '[' without a matching ']'.".to_string())
    } else {
        Ok(instructions)
    }
}

fn run(instructions: Vec<Instructions>, data_array: &mut [u8; 30000]) {
    let mut data_ptr = 0;
    let mut instruction_ptr = 0;
    
    while instruction_ptr < instructions.len() {
        let instruction = &instructions[instruction_ptr];

        match instruction {
            Instructions::IncrementPtr => {
                data_ptr = (data_ptr + 1) % data_array.len();
                instruction_ptr += 1;
            }
            Instructions::DecrementPtr => {
                data_ptr = (data_ptr + data_array.len() - 1) % data_array.len();
                instruction_ptr += 1;
            }
            Instructions::IncrementByte => {
                data_array[data_ptr] = data_array[data_ptr].wrapping_add(1);
                instruction_ptr += 1;
            }
            Instructions::DecrementByte => {
                data_array[data_ptr] = data_array[data_ptr].wrapping_sub(1);
                instruction_ptr += 1;
            }
            Instructions::OutputByte => {
                print!("{}", data_array[data_ptr] as char);
                instruction_ptr += 1;
            }
            Instructions::InputByte => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("Failed to read input");
                data_array[data_ptr] = input[0];
                instruction_ptr += 1;
            }
            Instructions::BeginLoop => {
                if data_array[data_ptr] == 0 {
                    // Skip to the matching ']' if the current byte is zero
                    let mut loop_depth = 1;
                    let mut temp_ptr = instruction_ptr+1;
                    while loop_depth > 0 && temp_ptr < instructions.len() {
                        match &instructions[temp_ptr] {
                            Instructions::BeginLoop => loop_depth += 1,
                            Instructions::EndLoop => loop_depth -= 1,
                            _ => {}
                        }
                        if loop_depth == 0 {
                            break;
                        }
                        temp_ptr += 1;
                    }
                    instruction_ptr = temp_ptr;
                } else {
                    instruction_ptr += 1;
                }
            }
            Instructions::EndLoop => {
                if data_array[data_ptr] != 0 {
                    // Go back to the matching '[' if the current byte is non-zero
                    let mut loop_depth = 1;
                    let mut temp_ptr = instruction_ptr - 1;
                    while loop_depth > 0 && temp_ptr > 0 {
                        match &instructions[temp_ptr] {
                            Instructions::EndLoop => loop_depth += 1,
                            Instructions::BeginLoop => loop_depth -= 1,
                            _ => {}
                        }
                        if loop_depth == 0 {
                            break;
                        }
                        temp_ptr -= 1;
                    }
                    instruction_ptr = temp_ptr;
                } else {
                    instruction_ptr += 1;
                }
            }
        }
    }
}

fn main() {
    let input = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]
    <+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]
    <+.<.+++.------.--------.>>>++++[<++++++++>-]<+."; // Hello World!
    let instructions_result = parse_instructions(input);

    match instructions_result {
        Ok(instructions) => {
            let mut data_array: [u8; 30000] = [0; 30000];
            run(instructions, &mut data_array);
            println!("\nMemory dump:");
            for (i, byte) in data_array.iter().enumerate() {
                if byte != &0 {
                    println!("[{}]: {}", i, byte);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
