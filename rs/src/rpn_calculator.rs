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

const PROMPT: &'static str = "> ";

const SPACE: u8 = b" "[0];
const TAB: u8 = b"\t"[0];
const LINE_FEED: u8 = b"\n"[0];
const ZERO: u8 = b"0"[0];
const NINE: u8 = b"9"[0];

pub fn run_repl<T: Read<u8> + Write<u8>>(port: &mut T) {
    let mut terms: [Term; MAX_TERMS] = [Term::Value(0); MAX_TERMS];
    loop {
        print_prompt(port);
        match read_expression(port, &mut terms).and_then(|n| evaluate(&terms, n)) {
            Ok(result) => print_result(result, port),
            Err(error) => print_error(error, port),
        }
    }
}

fn print_prompt(output: &mut impl Write<u8>) {
    for byte in PROMPT.as_bytes() {
        block!(output.write(*byte)).unwrap_or_default();
    }
}

fn read_expression(
    input: &mut impl Read<u8>,
    terms: &mut [Term; MAX_TERMS],
) -> Result<usize, Error> {
    let mut token: [u8; MAX_TOKEN_LENGTH] = [0; MAX_TOKEN_LENGTH];
    let mut number_of_terms = 0;
    loop {
        let (token_length, end_of_line) = read_token(input, &mut token)?;
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
    todo!();
}

fn print_error(error: Error, output: &mut impl Write<u8>) {
    todo!();
}

fn read_token(
    input: &mut impl Read<u8>,
    token: &mut [u8; MAX_TOKEN_LENGTH],
) -> Result<(usize, bool), Error> {
    let mut token_length = 0;
    loop {
        let byte = block!(input.read()).map_err(|_| Error::ReadError)?;
        if byte == SPACE || byte == TAB {
            return Ok((token_length, false));
        } else if byte == LINE_FEED {
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
