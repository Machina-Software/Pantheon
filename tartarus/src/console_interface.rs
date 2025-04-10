use std::collections::HashSet;

use crate::console_lib;
use crate::SharedState;
use rustyline::hint::Hint;
use rustyline::hint::Hinter;
use rustyline::Context;
use rustyline::Helper;
use rustyline::{error::ReadlineError, history::FileHistory, Editor};
use rustyline::{Completer, Highlighter, Validator};

use talaria::console::*;

#[derive(Completer, Helper, Validator, Highlighter)]
struct DIYHinter {}

#[derive(Hash, Debug, PartialEq, Eq)]
struct CommandHint {
    display: String,
    complete_up_to: usize,
}

impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.display[..self.complete_up_to])
        } else {
            None
        }
    }
}

impl CommandHint {
    fn new(line: &str, text: &str) -> Self {
        Self {
            display: text.into(),
            complete_up_to: line.len(),
        }
    }

    fn suffix(&self, strip_chars: usize) -> Self {
        Self {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}

impl Hinter for DIYHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<CommandHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }
        let console = Console::new(None);

        match console.auto_complete(line.to_string()) {
            Some(complete) => Some(CommandHint::new(line, &complete.replacen(line, "", 1))),
            None => None,
        }
    }
}

pub async fn start_console(shared_state: SharedState) {
    let mut rl = Editor::<DIYHinter, FileHistory>::new().unwrap();
    let h = DIYHinter {};
    rl.set_helper(Some(h));
    // Load command history if it exists
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    // rl.set_completion_ty;

    let mut console = Console::new(None);

    // print!("\x1B[2J\x1B[1;1H");
    println!("-----------------------------------");
    println!("Type 'help' for a list of commands.");

    loop {
        let readline = rl.readline(&console.status_line());
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let command_str = line.trim();
                let command = console.handle_command(command_str.to_string());
                match command {
                    Ok(command) => {
                        let response = console_lib::evaluate_command(
                            shared_state.clone(),
                            CommandContext {
                                command,
                                current_target: console.get_target(),
                            },
                        )
                        .await;

                        match response {
                            Ok(response) => {
                                match response.new_target {
                                    NewTarget::NoTarget => console.set_target(None),
                                    NewTarget::Target { ref target } => {
                                        console.set_target(Some(target.clone()))
                                    }
                                    NewTarget::NoChange => {}
                                }

                                println!("{}", response.output);
                            }
                            Err(err) => {
                                println!("{}", err.message);
                            }
                        }
                    }
                    Err(error) => println!("{}", error.to_string()),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C detected. Exiting.");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D detected. Exiting.");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history("history.txt").unwrap_or_else(|err| {
        eprintln!("Failed to save history: {:?}", err);
    });
}
