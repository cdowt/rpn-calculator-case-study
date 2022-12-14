use embedded_hal::serial::*;
use nb::block;

#[derive(Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Remainder,
}

#[derive(Clone, Copy)]
enum Term {
    Value(isize),
    Operator(Operator),
}

enum Error {
    InvalidTerm,
    TooManyTerms,
    ReadError,
    TokenTooLong,
    StackOverflow,
    StackUnderflow,
}

struct Stack {
    values: [isize; STACK_SIZE],
    top_index: usize,
}

impl Default for Stack {
    fn default() -> Self {
        Stack {
            values: [0; STACK_SIZE],
            top_index: 0,
        }
    }
}

impl Stack {
    fn push(&mut self, value: isize) -> Result<(), Error> {
        if self.top_index == STACK_SIZE {
            Err(Error::StackOverflow)
        } else {
            self.values[self.top_index] = value;
            self.top_index += 1;
            Ok(())
        }
    }

    fn pop(&mut self) -> Result<isize, Error> {
        if self.top_index == 0 {
            Err(Error::StackUnderflow)
        } else {
            self.top_index -= 1;
            Ok(self.values[self.top_index])
        }
    }
}

const MAX_TERMS: usize = 32;
const MAX_TOKEN_LENGTH: usize = 16;
const STACK_SIZE: usize = 32;
const MAX_DIGITS: usize = 20;

const PROMPT: &'static str = "> ";

const SPACE: u8 = b" "[0];
const TAB: u8 = b"\t"[0];
const CARRIAGE_RETURN: u8 = b"\r"[0];
const LINE_FEED: u8 = b"\n"[0];
const ZERO: u8 = b"0"[0];
const NINE: u8 = b"9"[0];

pub fn run_repl<T: Read<u8> + Write<u8>>(port: &mut T) {
    let mut terms: [Term; MAX_TERMS] = [Term::Value(0); MAX_TERMS];
    loop {
        print(PROMPT, port);
        match read_expression(port, &mut terms).and_then(|n| evaluate(&terms, n)) {
            Ok(result) => print_result(result, port),
            Err(error) => print_error(error, port),
        }
    }
}

fn read_expression<T: Read<u8> + Write<u8>>(
    port: &mut T,
    terms: &mut [Term; MAX_TERMS],
) -> Result<usize, Error> {
    let mut token: [u8; MAX_TOKEN_LENGTH] = [0; MAX_TOKEN_LENGTH];
    let mut number_of_terms = 0;
    loop {
        let (token_length, end_of_line) = read_token(port, &mut token)?;
        let term = read_operator(&token, token_length)
            .or(read_value(&token, token_length))
            .ok_or(Error::InvalidTerm)?;

        if number_of_terms == MAX_TERMS {
            return Err(Error::TooManyTerms);
        } else {
            terms[number_of_terms] = term;
            number_of_terms += 1;
        }

        if end_of_line {
            break;
        }
    }
    Ok(number_of_terms)
}

fn evaluate(terms: &[Term; MAX_TERMS], number_of_terms: usize) -> Result<isize, Error> {
    let mut stack: Stack = Default::default();
    for index in 0..number_of_terms {
        match terms[index] {
            Term::Value(value) => stack.push(value)?,
            Term::Operator(operator) => {
                let second = stack.pop()?;
                let first = stack.pop()?;
                let result = apply(operator, first, second);
                stack.push(result)?;
            }
        };
    }
    stack.pop()
}

fn print_result(result: isize, output: &mut impl Write<u8>) {
    let mut remaining: usize = if result < 0 {
        print("-", output);
        (result * -1) as usize
    } else {
        result as usize
    };

    let mut digits: [u8; MAX_DIGITS] = [0; MAX_DIGITS];
    let mut number_of_digits = 0;
    loop {
        let digit_value = remaining % 10;
        digits[number_of_digits] = ZERO + digit_value as u8;
        remaining /= 10;
        number_of_digits += 1;

        if remaining == 0 {
            break;
        }
    }

    for index in (0..number_of_digits).rev() {
        print_byte(digits[index], output);
    }
    end_line(output);
}

fn print_error(error: Error, output: &mut impl Write<u8>) {
    let message = match error {
        Error::InvalidTerm => "a term in the expression was invalid",
        Error::TooManyTerms => "too many terms in the expression",
        Error::ReadError => "error while reading input",
        Error::TokenTooLong => "a term in the expression was too long",
        Error::StackOverflow => "the stack overflowed",
        Error::StackUnderflow => "the stack underflowed",
    };

    end_line(output);
    print("Error: ", output);
    print(message, output);
    end_line(output);
}

fn read_token<T: Read<u8> + Write<u8>>(
    port: &mut T,
    token: &mut [u8; MAX_TOKEN_LENGTH],
) -> Result<(usize, bool), Error> {
    let mut token_length = 0;
    loop {
        let byte = echoing_read(port)?;

        if byte == SPACE || byte == TAB {
            return Ok((token_length, false));
        } else if byte == CARRIAGE_RETURN {
            print_byte(LINE_FEED, port);
            return Ok((token_length, true));
        } else if token_length == MAX_TOKEN_LENGTH {
            return Err(Error::TokenTooLong);
        } else {
            token[token_length] = byte;
            token_length += 1;
        }
    }
}

fn read_operator(token: &[u8; MAX_TOKEN_LENGTH], token_length: usize) -> Option<Term> {
    if token_length > 1 {
        None
    } else {
        match token[0] as char {
            '+' => Some(Operator::Plus),
            '-' => Some(Operator::Minus),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            '%' => Some(Operator::Remainder),
            _ => None,
        }
        .map(|operator| Term::Operator(operator))
    }
}

fn read_value(token: &[u8; MAX_TOKEN_LENGTH], token_length: usize) -> Option<Term> {
    let (sign, digits_start) = if token[0] as char == '-' {
        (-1, 1)
    } else {
        (1, 0)
    };

    let mut magnitude: isize = 0;
    for index in digits_start..token_length {
        if token[index] < ZERO || token[index] > NINE {
            return None;
        }
        magnitude *= 10;
        magnitude += (token[index] - ZERO) as isize;
    }

    Some(Term::Value(sign * magnitude))
}

fn apply(operator: Operator, first: isize, second: isize) -> isize {
    match operator {
        Operator::Plus => first + second,
        Operator::Minus => first - second,
        Operator::Multiply => first * second,
        Operator::Divide => first / second,
        Operator::Remainder => first % second,
    }
}

fn print(message: &str, output: &mut impl Write<u8>) {
    for &byte in message.as_bytes() {
        print_byte(byte, output);
    }
}

fn print_byte(byte: u8, output: &mut impl Write<u8>) {
    block!(output.write(byte)).unwrap_or_default();
}

fn end_line(output: &mut impl Write<u8>) {
    print_byte(CARRIAGE_RETURN, output);
    print_byte(LINE_FEED, output);
}

fn echoing_read<T: Read<u8> + Write<u8>>(port: &mut T) -> Result<u8, Error> {
    let byte = block!(port.read()).map_err(|_| Error::ReadError)?;
    print_byte(byte, port);
    Ok(byte)
}
