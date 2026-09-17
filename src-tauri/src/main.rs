// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use rustyline::{
    Editor, Helper, Highlighter, Validator,
    completion::Completer,
    error::ReadlineError,
    hint::{Hinter, HistoryHinter},
};

use gpostman_lib::{commons::{AppState, LayerSourceCollection}, link_engine::LinkEngine};

#[derive(Helper, Validator, Highlighter)]
struct ShellHelper {
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
}

impl Completer for ShellHelper {
    type Candidate = String;

    fn complete(
        &self, // FIXME should be `&mut self`
        line: &str,
        pos: usize,
        _: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let commands = vec!["help", "add", "remove", "exit"];
        let mut matches = Vec::new();

        let current = &line[..pos];

        for cmd in commands {
            if cmd.starts_with(current) {
                matches.push(cmd.to_string());
            }
        }

        Ok((0, matches))
    }
}

impl Hinter for ShellHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        self.hinter.hint(line, pos, ctx)
    }
}

static VIEWPORT_OPEN: AtomicBool = AtomicBool::new(false);

fn open_viewport(lsc: Arc<Mutex<HashMap<String, LayerSourceCollection>>>, link_e: Arc<tokio::sync::Mutex<LinkEngine>>) {
    if !VIEWPORT_OPEN.swap(true, Ordering::Relaxed) {
        gpostman_lib::run(AppState {
            lsc: lsc,
            d_lsc: None,
            l_engine: link_e,
            sender: Mutex::new(None)
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let config = rustyline::Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .build();

    let mut rl: Editor<ShellHelper, _> = Editor::with_config(config).map_err(|_| ())?;

    rl.set_helper(Some(ShellHelper {
        hinter: HistoryHinter::new(),
    }));

    let l_engine = Arc::new(tokio::sync::Mutex::new(LinkEngine::new()));
    let collection = l_engine.lock().await.collection.clone();
    let l_engine_c = l_engine.clone();
    let rt_handle = tokio::runtime::Handle::current();

    std::thread::spawn(async move || {
        loop {
            let readline = rl.readline("geoman>> ");

            match readline {
                Ok(line) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    let _ = rl.add_history_entry(trimmed);

                    let mut parts = trimmed.split_whitespace();
                    let command = parts.next().unwrap_or("");
                    let args: Vec<&str> = parts.collect();

                    match command {
                        "exit" | "quit" => {
                            println!("Exiting...");
                            std::process::exit(0);
                        }
                        "add" => {
                            if let Some(first) = args.get(0) {
                                let slug_id = rt_handle.block_on(l_engine.lock().await.add_url(first, None));
                                println!("Added successfully with slug id: {}", slug_id);
                            } else {
                                println!("Missing the url parameter, type 'help' to see commands.");
                            }
                        }
                        "remove" => {
                            if let Some(first) = args.get(0) {
                                match l_engine.lock().await.remove(first.to_string()) {
                                    Ok(_) => {println!("Removed the url successfully.");}
                                    Err(_) => {println!("Invalid url or slug id")}
                                }
                            } else {
                                println!(
                                    "Missing the url or slug_id parameter, type 'help' to see commands."
                                );
                            }
                        }
                        "help" => {
                            println!("Available commands:");
                            println!("  add <url>    - Adds an endpoint");
                            println!("  remove <url|slug_id> - Removes an endpoint");
                            println!("  help         - Shows this message");
                            println!("  exit         - Exits the application");
                        }
                        _ => {
                            println!("Unknown command: '{}'. Type 'help' for options.", command);
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Ctrl+C received. Exiting...");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("Ctrl+D received. Exiting...");
                    break;
                }
                Err(err) => {
                    println!("Error reading line: {:?}", err);
                    break;
                }
            }
        }
    });

    open_viewport(collection, l_engine_c);

    Ok(())
}
