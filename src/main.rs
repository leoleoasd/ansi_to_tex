use std::collections::VecDeque;
use std::error::Error;
use std::io::{stdin, Read};

use ansi_parser::{Output, AnsiParser};
use ansi_parser::AnsiSequence;

enum Stack {
    Bold,
    Color
}
fn main() -> Result<(), Box<dyn Error>> {
    let ansi_colors = vec![
        "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"
    ];
    let mut input: String = "".to_string();
    stdin().read_to_string(&mut input)?;
    let parsed = input
        .ansi_parse();
    let mut stack: VecDeque<Stack> = VecDeque::new();
    println!(r"\begin{{Verbatim}}[commandchars=+\[\]]");
    for p in parsed {
        match p {
            Output::TextBlock(t) => {
                print!("{t}");
            },
            Output::Escape(t) => {
                match t {
                    AnsiSequence::SetGraphicsMode(v) => {
                        for i in v {
                            match i {
                                1 => {
                                    stack.push_back(Stack::Bold);
                                    print!("+begingroup[]+bfseries[]")
                                }
                                0 => {
                                    while let Some(x) = stack.pop_back() {
                                        match x {
                                            Stack::Bold => {
                                                print!("+endgroup[]")
                                            }
                                            Stack::Color => {
                                                print!("+endgroup[]")
                                            }
                                        }
                                    }
                                }
                                30..=37 => {
                                    stack.push_back(Stack::Color);
                                    print!("+begingroup[]+color[ANSI{}]", ansi_colors[(i-30) as usize])
                                }
                                _ => todo!()
                            }
                        }
                    }
                    _ => todo!()
                }
            },
        }
    }
    
    while let Some(x) = stack.pop_back() {
        match x {
            Stack::Bold => {
                print!("+endgroup[]")
            }
            Stack::Color => {
                print!("+endgroup[]")
            }
        }
    }
    println!(r"\end{{Verbatim}}");
    Ok(())
}
