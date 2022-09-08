use embedded_hal::serial::*;
use nb::block;

#[derive(Clone, Copy)]
enum Term {
    Value(isize),
}

enum Error {
}

const MAX_TERMS: usize = 32;

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
    todo!();
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
