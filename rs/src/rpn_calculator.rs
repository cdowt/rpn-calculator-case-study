use embedded_hal::serial::*;
use nb::block;

#[derive(Clone, Copy)]
enum Term {
    Value(isize),
}

enum Error {
    InvalidTerm,
    TooManyTerms,
}

const MAX_TERMS: usize = 32;
const MAX_TOKEN_LENGTH: usize = 16;

const PROMPT: &'static str = "> ";

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
    todo!();
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
    todo!();
}

fn read_operator(token: &[u8; MAX_TOKEN_LENGTH], token_length: usize) -> Option<Term> {
    todo!();
}

fn read_value(token: &[u8; MAX_TOKEN_LENGTH], token_length: usize) -> Option<Term> {
    todo!();
}
