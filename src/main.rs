use clap::Parser;
use std::io::Write;

mod cli;
mod data;
mod things;

use cli::WannaCli;
use things::{Thing, Things};

fn confirm_similar_name(candidate: &str) -> anyhow::Result<bool> {
    print!("Did you mean \"{candidate}\"? (y): ");
    std::io::stdout().flush()?;

    let mut ans = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut ans)?;
    let ans = ans.trim();

    Ok(ans.len() == 0 || ans.to_lowercase() == "y")
}

fn run() -> anyhow::Result<()> {
    let args = WannaCli::parse();

    // Load known things.
    let things_path = data::establish_things_file()?;
    let mut things = Things::from_file(&things_path)?;

    if let Some(verb) = args.verb.as_deref() {
        match verb {
            // "wanna do ..."
            "do" => {
                if let Some(name) = args.name.as_deref() {
                    match name {
                        // Get a random thing.
                        // TODO: How to best implement modifiers like "wanna do something after <_>"?
                        "something" => {
                            if let Some(thing) = things.get_something() {
                                println!("You should {thing}!");
                            } else {
                                println!("Tell me some things you wanna do first.");
                            }
                        }

                        // User needs to provide a categorizing verb for their thing.
                        _ => println!("You've gotta be more specific."),
                    }
                } else {
                    println!("What do you wanna do?");
                }
            }
            "mark" => todo!(),
            "finish" => todo!(),
            "forget" => todo!(),

            // "wanna <_> ..."
            _ => {
                // Ex. "wanna read" or "wanna read something".
                if args.name.is_none() || args.name.as_deref().is_some_and(|n| n == "something") {
                    if let Some(thing) = things.get_something_for_verb(verb) {
                        println!("You should {thing}!");
                    } else {
                        println!("Tell me some things you wanna {verb} first.");
                    }
                } else {
                    // args.name cannot be None here.
                    let name = args.name.as_deref().unwrap();

                    if let Some(thing) = things.find_existing_thing(name) {
                        println!("You already wanna {thing}.");
                    } else if let Some(thing) = things.find_similar_candidate(name) {
                        if confirm_similar_name(&thing.name)? {
                            println!("You already wanna {thing}.");
                        } else {
                            things.add_something(Thing::new(verb, name));
                        }
                    } else {
                        things.add_something(Thing::new(verb, name));
                    }
                }
            }
        }
    } else {
        // Args are parsed positionally, so no verb means no args.
        println!("What do you wanna do?");
    }

    // Save known things.
    things.save_to_file(things_path)?;
    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(error) => {
            eprintln!("Error: {:?}", error);
            1
        }
    });
}
