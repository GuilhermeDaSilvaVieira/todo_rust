//! # Console Todo App
//! Organize your live with this simple, fast, and reliable console todo app
//!
//! # Features
//!

use std::{
    fs::File,
    io::{self, BufRead, Read, Write},
};

enum Command {
    All,
    Add(String),
    Finish(u32),
    Clear,
    Help,
    Save,
    Quit,
}

impl Command {
    fn from_input(input: &str) -> Option<Self> {
        let words: Vec<&str> = input.trim().split_whitespace().collect();
        let parameter = words[1..].join(" ").to_string();
        match words[0].to_uppercase().trim() {
            "ALL" => Some(Command::All),
            "ADD" if !parameter.is_empty() => Some(Command::Add(parameter)),
            "FINISH" if !parameter.is_empty() && Command::is_numeric(&parameter) => {
                Some(Command::Finish(parameter.parse().unwrap()))
            }
            "CLEAR" => Some(Command::Clear),
            "HELP" => Some(Command::Help),
            "SAVE" => Some(Command::Save),
            "QUIT" => Some(Command::Quit),
            _ => None,
        }
    }

    fn save(todos: &Todo) -> std::io::Result<()> {
        let mut file = File::create("todo_rust.conf")?;
        for (task, _) in todos {
            file.write_all(format!("{}\n", task).as_bytes())?;
        }

        Ok(())
    }

    fn help() {
        println!();
        println!("Type 'ALL' to list all tasks");
        println!("Type 'ADD <task>' to add a task");
        println!("Type 'FINISH <id>' finishes a task");
        println!("Type 'CLEAR' to clear the screen");
        println!("Type 'HELP' to show this message");
        println!("Type 'SAVE' to save all tasks");
        println!("Type 'QUIT' to exit");
        println!();
    }

    fn is_numeric(s: &String) -> bool {
        for c in s.chars() {
            if !c.is_numeric() {
                return false;
            }
        }
        true
    }
}

type Todo = Vec<(String, u32)>;

pub struct Config {
    todos: Todo,
    id: u32,
}

impl Config {
    pub fn new() -> Config {
        if let Ok(mut file) = File::open("todo_rust.conf") {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("error: can't read file");
            let mut id = 0;
            let todos: Todo = contents
                .lines()
                .map(|x| {
                    id += 1;
                    (x.to_string(), id)
                })
                .collect();
            Config { todos, id }
        } else {
            Config {
                todos: Todo::new(),
                id: 0,
            }
        }
    }

    pub fn run(mut self) -> std::io::Result<()> {
        Command::help();
        for line in io::stdin().lock().lines() {
            let input = line.expect("Unable to read user input");
            match Command::from_input(&input) {
                Some(Command::All) => {
                    println!("<=============================================>");
                    for (task, id) in &self.todos {
                        println!("{}. {}", id, task);
                    }
                }
                Some(Command::Add(task)) => {
                    self.id += 1;
                    self.todos.push((task, self.id));
                }
                Some(Command::Finish(id)) => {
                    if self.todos.len() >= id as usize {
                        self.todos.retain(|(_, y)| y != &id);
                        self.id -= 1;

                        // Reorder todos
                        let x = self.todos.clone();
                        let mut y = 0;
                        for (x, _) in x {
                            self.todos[y] = (x, (y + 1) as u32);
                            y += 1;
                        }
                    } else {
                        eprintln!("Can't finish what doesn't exist");
                    }
                }
                Some(Command::Clear) => terminal::stdout()
                    .act(terminal::Action::ClearTerminal(terminal::Clear::All))
                    .unwrap(),
                Some(Command::Help) => Command::help(),
                Some(Command::Save) => {
                    Command::save(&self.todos)?;
                }
                Some(Command::Quit) => break,
                None => eprintln!("Input error!"),
            }
            println!();
        }

        Ok(())
    }
}
