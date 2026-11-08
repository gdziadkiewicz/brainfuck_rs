//create error type for my lib and use it for brain_luck function
use std::error::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BrainLuckError {
    UnexpectedCharInCode(char),
    UnexpectedEndOfInput,
    UnbalancedBrackets,
    MemoryPointerOutOfBounds,
}

impl std::fmt::Display for BrainLuckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            BrainLuckError::UnexpectedCharInCode(c) => {
                format!("unexpected char {} occured in code.", c)
            }
            BrainLuckError::UnexpectedEndOfInput => String::from("unexpected end of input."),
            BrainLuckError::UnbalancedBrackets => String::from("unbalanced brackets."),
            BrainLuckError::MemoryPointerOutOfBounds => {
                String::from("memory pointer out of bounds.")
            }
        };
        f.write_str(&message)
    }
}

impl Error for BrainLuckError {}

pub fn brain_luck(code: &str, input: Vec<u8>) -> Result<Vec<u8>, BrainLuckError> {
    let mut output = Vec::new();
    let mut input = input.iter();
    let mut memory = vec![0u8; 1000];
    let mut mem_ptr: usize = 0;
    let mut ins_ptr = 0;

    let insts = code.chars().collect::<Vec<_>>();
    while let Some(c) = insts.get(ins_ptr) {
        match c {
            '>' => {
                if mem_ptr == memory.len() - 1 {
                    return Err(BrainLuckError::MemoryPointerOutOfBounds);
                }
                mem_ptr += 1;
            }
            '<' => {
                mem_ptr = mem_ptr
                    .checked_sub(1)
                    .ok_or(BrainLuckError::MemoryPointerOutOfBounds)?
            }
            '+' => memory[mem_ptr] = memory[mem_ptr].wrapping_add(1),
            '-' => memory[mem_ptr] = memory[mem_ptr].wrapping_sub(1),
            '.' => output.push(memory[mem_ptr]),
            ',' => memory[mem_ptr] = *input.next().ok_or(BrainLuckError::UnexpectedEndOfInput)?,
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
    fn echo_until_byte_255_encountered() -> Result<(), BrainLuckError> {
        // Echo until byte 255 encountered
        assert!(brain_luck(",+[-.,+]", [b"Codewars".as_slice(), &[255]].concat())? == b"Codewars");
        Ok(())
    }

    #[test]
    fn echo_until_byte_0_encountered() -> Result<(), BrainLuckError> {
        // Echo until byte 0 encountered
        assert!(brain_luck(",[.[-],]", b"Codewars\0".to_vec())? == b"Codewars");
        Ok(())
    }

    #[test]
    fn multiply_two_numbers() -> Result<(), BrainLuckError> {
        // Multiply two numbers
        for a in (0u8..=15).chain([127, 255]) {
            for b in (0u8..=15).chain([127, 255]) {
                assert!(
                    brain_luck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![a, b])?
                        == vec![a.wrapping_mul(b)]
                );
            }
        }
        Ok(())
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
    fn add_two_numbers() -> Result<(), BrainLuckError> {
        // Add two numbers
        for a in 0u8..=255 {
            for b in 0u8..=255 {
                assert!(brain_luck(",>,<[->+<]>.", vec![a, b])? == vec![a.wrapping_add(b)]);
            }
        }
        Ok(())
    }

    #[test]
    fn subtract_two_numbers() -> Result<(), BrainLuckError> {
        // Subtract two numbers
        for a in (0u8..=15).chain([127, 255]) {
            for b in (0u8..=15).chain([127, 255]) {
                assert!(brain_luck(",>,[<->-]<.", vec![a, b])? == vec![a.wrapping_sub(b)]);
            }
        }
        Ok(())
    }

    #[test]
    fn memory_pointer_bounds() {
        assert!(brain_luck("<", vec![]) == Err(BrainLuckError::MemoryPointerOutOfBounds));
        assert!(
            brain_luck(&">".repeat(1000), vec![]) == Err(BrainLuckError::MemoryPointerOutOfBounds)
        );
        assert!(brain_luck(&format!("{}+.", ">".repeat(999)), vec![]) == Ok(vec![1]));
    }

    #[test]
    fn matching_closing_bracket_ptr_balanced() -> Result<(), BrainLuckError> {
        // Balanced brackets
        let code = "++[--[++]]";
        let insts = code.chars().collect::<Vec<_>>();
        assert!(matching_closing_bracket_ptr(&insts, 2, Bracket::LBracket)? == 9);
        assert!(matching_closing_bracket_ptr(&insts, 9, Bracket::RBracket)? == 2);
        Ok(())
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
