pub fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    let mut input = input.iter();
    let mut memory = vec![0u8; 1000];
    let mut mem_ptr = 0;
    let mut ins_ptr = 0;

    let insts = code.chars().collect::<Vec<_>>();
    while let Some(c) = insts.get(ins_ptr) {
        match c {
            '>' => {
                mem_ptr += 1;
                ins_ptr += 1;
            }
            '<' => {
                mem_ptr -= 1;
                ins_ptr += 1;
            }
            '+' => {
                memory[mem_ptr] = memory[mem_ptr].wrapping_add(1);
                ins_ptr += 1;
            }
            '-' => {
                memory[mem_ptr] = memory[mem_ptr].wrapping_sub(1);
                ins_ptr += 1;
            }
            '.' => {
                output.push(memory[mem_ptr]);
                ins_ptr += 1;
            }
            ',' => {
                let b = input.next().unwrap();
                memory[mem_ptr] = *b;
                ins_ptr += 1;
            }
            '[' => {
                let b = memory[mem_ptr];
                if b == 0 {
                    //jump to next after closing bracket
                    ins_ptr = after_matching_closing_bracket(&insts, ins_ptr, Bracket::LBracket);
                } else {
                    ins_ptr += 1;
                }
            }
            ']' => {
                let b = memory[mem_ptr];
                if b != 0 {
                    //jump to previous after closing bracket
                    ins_ptr = after_matching_closing_bracket(&insts, ins_ptr, Bracket::RBracket);
                } else {
                    ins_ptr += 1;
                }
            }
            unexpected_char => panic!("Unexpected char {} occured in code.", unexpected_char),
        }
    }
    output
}

enum Bracket {
    LBracket,
    RBracket,
}

macro_rules! add {
    ($u:ident,$s:ident) => {
        if $s < 0 { $u - (-$s) as usize} else { $u + $s as usize}
    };
}

fn after_matching_closing_bracket(insts: &Vec<char>, ins_ptr: usize, b: Bracket) -> usize {
    let direction = match b {
        Bracket::LBracket => 1,
        Bracket::RBracket => -1,
    };
    
    let mut counter: i32 = 0;
    let mut ptr = ins_ptr;
    loop {
        let c = insts.get(ptr);
        match c {
            None => panic!("Unbalanced brackets(EOF)!"),
            Some('[') => counter += direction,
            Some(']') => counter -= direction,
            Some(_) => (),
        }
        if counter < 0 {
            panic!("Unbalanced brackets!")
        }
        if counter == 0 {
            return ptr;
        }
        ptr = add!(ptr, direction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;

    #[test]
    fn echo_until_byte_255_encountered() {
        // Echo until byte 255 encountered
        assert!(String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap() == "Codewars");
    }

    #[test]
    fn echo_until_byte_0_encountered() {
        // Echo until byte 0 encountered
        assert!(String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0))).unwrap() == "Codewars");
    }

    #[test]
    fn multiply_two_numbers() {
        // Multiply two numbers
        assert!(brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]) == vec![72]);
    }
    
    // Takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
    // Without it, character-based tests are a pain   
    fn ez_vec(s: &str, i: u8) -> Vec<u8> {
      let mut v = s.to_string().into_bytes();
      v.push(i);
      v
    }   
}