use clap::Parser;
use clap::Subcommand;
use std::cell::RefCell;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;

use lox_interpreter as lox;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Tokenize { filename: PathBuf },
    Parse { filename: PathBuf },
    Evaluate { filename: PathBuf },
    Run { filename: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Tokenize { filename } => {
            let file_content = fs::read_to_string(&filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename.display()).unwrap();
                String::new()
            });

            let file_content = dbg!(file_content);
            let scanner = lox::Scanner::new(&file_content);
            let mut success = true;
            for token in scanner.into_iter() {
                match token {
                    Ok(token) => println!("{token}"),
                    Err(error) => {
                        success = false;
                        eprintln!("{}", error.to_string());
                    }
                }
            }

            if !success {
                exit(65)
            }
        }
        Command::Parse { filename } => {
            let file_content = fs::read_to_string(&filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename.display()).unwrap();
                String::new()
            });

            let file_content = dbg!(file_content);

            {
                use lox::ci::Debuge;

                let scanner = lox::Scanner::new(&file_content);
                let mut parser = lox::ci::Parser::new(&file_content, scanner);

                match parser.parse_expression() {
                    Ok(result) => println!("{s}", s = result.print()),
                    Err(error) => {
                        eprintln!("{error}");
                        exit(65);
                    }
                };
            }
        }
        Command::Evaluate { filename } => {
            let file_content = fs::read_to_string(&filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename.display()).unwrap();
                String::new()
            });

            let scanner = lox::Scanner::new(&file_content);

            {
                use lox::ci::Debuge;
                use lox::ci::Interpret;

                let mut parser = lox::ci::Parser::new(&file_content, scanner);
                match parser.parse_expression() {
                    Ok(result) => {
                        dbg!(format!("{result}", result = result.print()));

                        match result
                            .interpret(Rc::new(RefCell::new(lox::ci::Environment::default())))
                        {
                            Ok(ev) => println!("{ev}"),
                            Err(error) => {
                                eprintln!("{error}");
                                exit(70);
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!("{error}");
                        exit(65);
                    }
                };
            }
        }
        Command::Run { filename } => {
            let file_content = fs::read_to_string(&filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename.display()).unwrap();
                String::new()
            });

            let scanner = lox::Scanner::new(&file_content);
            let parser = lox::ci::Parser::new(&file_content, scanner);

            let statements = parser.parse_statements().into_iter().collect::<Vec<_>>();

            let traverser = lox::ci::Traverser::new();
            match traverser.run(&statements) {
                Err(error) => {
                    eprintln!("{error}");
                    exit(65);
                }
                _ => {}
            }

            let mut interpreter = lox::ci::Interpreter::new(statements);
            interpreter.run();
        }
    }

    Ok(())
}
