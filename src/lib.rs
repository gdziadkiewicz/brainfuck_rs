//create error type for my lib and use it for brain_luck function
use std::error::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BrainLuckError {
    UnexpectedCharInCode(char),
    UnexpectedEndOfInput,
    UnbalancedBrackets,
}

impl std::fmt::Display for BrainLuckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            BrainLuckError::UnexpectedCharInCode(c) => {
                format!("unexpected char {} occured in code.", c)
            }
            BrainLuckError::UnexpectedEndOfInput => String::from("unexpected end of input."),
            BrainLuckError::UnbalancedBrackets => String::from("unbalanced brackets."),
        };
        f.write_str(&message)
    }
}

impl Error for BrainLuckError {}

pub fn brain_luck(code: &str, input: Vec<u8>) -> Result<Vec<u8>, BrainLuckError> {
    let mut output = Vec::new();
    let mut input = input.iter();
    let mut memory = vec![0u8; 1000];
    let mut mem_ptr = 0;
    let mut ins_ptr = 0;

    let insts = code.chars().collect::<Vec<_>>();
    while let Some(c) = insts.get(ins_ptr) {
        match c {
            '>' => mem_ptr += 1,
            '<' => mem_ptr -= 1,
            //TODO: add handling for mem_ptr going out of bounds (or accessing memory outside of bounds)
            '+' => memory[mem_ptr] = memory[mem_ptr].wrapping_add(1),
            '-' => memory[mem_ptr] = memory[mem_ptr].wrapping_sub(1),
            '.' => output.push(memory[mem_ptr]),
            ',' => {
                memory[mem_ptr] = *input
                    .next()
                    .ok_or(BrainLuckError::UnexpectedEndOfInput)?
            }
            '[' => {
                if memory[mem_ptr] == 0 {
                    ins_ptr = matching_closing_bracket_ptr(&insts, ins_ptr, Bracket::LBracket)?;
                }
            }
            ']' => {
                if memory[mem_ptr] != 0 {
                    ins_ptr = matching_closing_bracket_ptr(&insts, ins_ptr, Bracket::RBracket)?;
                }
            }
            c if c.is_whitespace() => (),
            unexpected_char => return Err(BrainLuckError::UnexpectedCharInCode(*unexpected_char)),
        }
        ins_ptr += 1;
    }
    Ok(output)
}

enum Bracket {
    LBracket,
    RBracket,
}

macro_rules! add {
    ($u:ident,$s:ident) => {
        if $s < 0 {
            $u.checked_sub((-$s) as usize)
        } else {
            Some($u + $s as usize)
        }
    };
}

fn matching_closing_bracket_ptr(
    insts: &[char],
    ins_ptr: usize,
    b: Bracket,
) -> Result<usize, BrainLuckError> {
    let direction = match b {
        Bracket::LBracket => 1,
        Bracket::RBracket => -1,
    };

    let mut counter: i32 = 0;
    let mut ptr = ins_ptr;
    loop {
        let c = insts.get(ptr);
        match c {
            None => return Err(BrainLuckError::UnbalancedBrackets),
            Some('[') => counter += direction,
            Some(']') => counter -= direction,
            Some(_) => (),
        }
        if counter < 0 {
            return Err(BrainLuckError::UnbalancedBrackets);
        }
        if counter == 0 {
            return Ok(ptr);
        }
        ptr = match add!(ptr, direction) {
            Some(p) => p,
            None if direction == -1 => return Err(BrainLuckError::UnbalancedBrackets),
            None => unreachable!(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;

    #[test]
    fn echo_until_byte_255_encountered() {
        // Echo until byte 255 encountered
        assert!(
            String::from_utf8(brain_luck(",+[-.,+]", ez_vec("Codewars", 255)).unwrap()).unwrap()
                == "Codewars"
        );
    }

    #[test]
    fn echo_until_byte_0_encountered() {
        // Echo until byte 0 encountered
        assert!(
            String::from_utf8(brain_luck(",[.[-],]", ez_vec("Codewars", 0)).unwrap()).unwrap()
                == "Codewars"
        );
    }

    #[test]
    fn multiply_two_numbers() {
        // Multiply two numbers
        assert!(brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]).unwrap() == vec![72]);
    }

    #[test]
    fn unexpected_char_in_code() {
        // Unexpected character in code
        assert!(matches!(
            brain_luck(",[.[-]a,]", ez_vec("Hello", 0)),
            Err(BrainLuckError::UnexpectedCharInCode('a'))
        ));
    }

    #[test]
    fn unbalanced_brackets() {
        // Unbalanced brackets
        assert!(
            brain_luck("[.[-],", ez_vec("Hello", 0)) == Err(BrainLuckError::UnbalancedBrackets)
        );
    }

    #[test]
    fn unexpected_end_of_input() {
        // Unexpected end of input
        assert!(matches!(
            brain_luck(",[.[-],]", vec![72, 101, 108, 108]),
            Err(BrainLuckError::UnexpectedEndOfInput)
        ));
    }

    #[test]
    fn add_two_numbers() {
        // Add two numbers
        assert!(brain_luck(",>,<[->+<]>.", vec![5, 10]).unwrap() == vec![15]);
    }

    #[test]
    fn subtract_two_numbers() {
        // Subtract two numbers
        assert!(brain_luck(">,>,[<[->]<]>>[[<+>-]>>]<<<.", vec![10, 5]).unwrap() == vec![5]);
    }

    #[test]
    fn matching_closing_bracket_ptr_balanced() {
        // Balanced brackets
        let code = "++[--[++]]";
        let insts = code.chars().collect::<Vec<_>>();
        assert!(matching_closing_bracket_ptr(&insts, 2, Bracket::LBracket).unwrap() == 9);
        assert!(matching_closing_bracket_ptr(&insts, 9, Bracket::RBracket).unwrap() == 2);
    }

    #[test]
    fn matching_closing_bracket_ptr_unbalanced_left() {
        // Unbalanced brackets (left)
        let code = "++[--[++]";
        let insts = code.chars().collect::<Vec<_>>();
        assert!(matches!(
            matching_closing_bracket_ptr(&insts, 2, Bracket::LBracket),
            Err(BrainLuckError::UnbalancedBrackets)
        ));
    }

    #[test]
    fn matching_closing_bracket_ptr_unbalanced_right() {
        // Unbalanced brackets (right)
        let code = "++[--[++]]]";
        let insts = code.chars().collect::<Vec<_>>();
        assert!(matches!(
            matching_closing_bracket_ptr(&insts, 10, Bracket::RBracket),
            Err(BrainLuckError::UnbalancedBrackets)
        ));
    }

    // Takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
    // Without it, character-based tests are a pain
    fn ez_vec(s: &str, i: u8) -> Vec<u8> {
        let mut v = s.to_string().into_bytes();
        v.push(i);
        v
    }
}
