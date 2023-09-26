use std::error::Error;
use std::io::{self, Write};
use std::process::exit;

mod todo;
use todo::*;
mod utils;
use utils::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut todos: Vec<Todo> = Vec::new();
    let mut scope: String = String::from("global");
    let mut prompt: String = String::from("> ");
    let mut current_todo_idx: usize = 0;
    let pad: String = "-".repeat(47);

    let usage = String::from(format!(
        "
              //!  [ todors v0.1 ]  !//
    {}
    [ Create new todo:                     create ]
    [ View all todos:                        list ]
    [ Select a todo:                  select <ID> ]
    [ Show todo information (select mode):   show ]
    [ Update a todo (select mode):         update ]
    [ Start progress on todo (select mode): start ]
    [ Finish a todo (select mode):       complete ]
    [ Delete a todo (select mode):         delete ]
    [ Display this help message:             help ]
    [ Exit the program:                      exit ]
    {}
    ",
        pad, pad
    ));
    println!("{}", usage);

    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read a line");

        let cmd: Vec<&str> = input.split_whitespace().collect();
        match cmd.first().unwrap_or(&"help") {
            &"create" => {
                print!("Title: ");
                io::stdout().flush().unwrap();
                let mut title: String = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read title");

                print!("Description: ");
                io::stdout().flush().unwrap();
                let mut description: String = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read description");

                io::stdout().flush().unwrap();

                let priority: Priority;
                loop {
                    let mut priority_str = String::from("");
                    print!("Priority: ");
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut priority_str)
                        .expect("Failed to read priority");
                    if PRIORITY_MAP.contains_key(&priority_str.trim().to_uppercase()) {
                        priority = PRIORITY_MAP
                            .get(&priority_str.trim().to_uppercase())
                            .unwrap()
                            .clone();
                        break;
                    }
                    println!(
                        "Priority {} not found! Select one from: {:#?}",
                        priority_str.trim(),
                        PRIORITY_MAP.keys()
                    );
                }

                let due_date: u32;
                loop {
                    print!("Due date: ");
                    let mut due_date_str: String = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut due_date_str)
                        .expect("Failed to read due_date");

                    match str_to_u32(due_date_str.trim()) {
                        Ok(v) => {
                            due_date = v;
                            break;
                        }
                        Err(e) => {
                            println!("Due date {} not valid! ({})", due_date_str.trim(), e);
                        }
                    }
                }

                let todo: Todo = Todo::new(
                    title.trim().to_string(),
                    description.trim().to_string(),
                    priority,
                    due_date,
                );
                println!("Todo created: {:#?}", todo);
                todos.push(todo);
            }
            &"list" => {
                for (i, todo) in todos.iter().enumerate() {
                    println!("[ TODO ID: {} ]", i);
                    println!("{:#?}", todo);
                    println!("{}", pad);
                }
            }
            &"select" => {
                if cmd.len() < 2 {
                    println!("Usage: select <ID>")
                } else {
                    match str_to_id(cmd[1]) {
                        Ok(todo_idx) => {
                            if todos.get(todo_idx).is_some() {
                                current_todo_idx = todo_idx;
                                prompt = format!("(TODO {})> ", current_todo_idx);
                                scope = String::from("select");
                            } else {
                                println!("Todo {} not found!", todo_idx);
                            }
                        }
                        Err(e) => {
                            println!("Todo {} not found! ({})", cmd[1], e);
                        }
                    }
                }
            }
            &"show" => {
                if scope != "select" {
                    println!("Not in select mode");
                } else {
                    println!("[ TODO: {} ]", current_todo_idx);
                    println!("{:?}", todos[current_todo_idx]);
                    println!("{}", pad);
                }
            }
            &"start" => {
                if scope != "select" {
                    println!("Not in select mode");
                } else {
                    todos[current_todo_idx].status = Status::InProgress;
                }
            }
            &"update" => {
                if scope != "select" {
                    println!("Not in select mode");
                } else {
                    print!("Title: ");
                    io::stdout().flush().unwrap();
                    let mut title: String = String::new();
                    io::stdin()
                        .read_line(&mut title)
                        .expect("Failed to read title");

                    print!("Description: ");
                    io::stdout().flush().unwrap();
                    let mut description: String = String::new();
                    io::stdin()
                        .read_line(&mut description)
                        .expect("Failed to read description");

                    print!("Priority: ");
                    io::stdout().flush().unwrap();
                    let mut priority_str: String = String::new();
                    let mut priority: Priority = todos[current_todo_idx].priority;
                    io::stdin()
                        .read_line(&mut priority_str)
                        .expect("Failed to read priority");
                    if priority_str.trim() != "" {
                        priority_str = priority_str.trim().to_string();
                        priority = PRIORITY_MAP
                            .get(&priority_str.to_uppercase())
                            .unwrap()
                            .clone();
                    } else {
                        println!("Priority not updated ({:#?})", priority);
                    }

                    print!("Due date: ");
                    io::stdout().flush().unwrap();
                    let mut due_date_str: String = String::new();
                    let mut due_date: u32 = todos[current_todo_idx].due_date;
                    io::stdin()
                        .read_line(&mut due_date_str)
                        .expect("Failed to read due date");
                    if due_date_str.trim() != "" {
                        due_date_str = due_date_str.trim().to_string();
                        due_date = str_to_u32(due_date_str.trim()).unwrap();
                    } else {
                        println!("Due date not updated ({})", due_date);
                    }
                    todos[current_todo_idx].update(title, description, priority, due_date);
                }
            }
            &"complete" => {
                if scope != "select" {
                    println!("Not in select mode");
                } else {
                    todos[current_todo_idx].status = Status::Completed;
                }
            }
            &"delete" => {
                if scope != "select" {
                    println!("Not in select mode");
                } else {
                    todos.remove(current_todo_idx);
                    println!("Todo {} removed!", current_todo_idx);
                    scope = String::from("global");
                    prompt = String::from("> ");
                    current_todo_idx = 0;
                }
            }
            &"back" => {
                if scope != "global" {
                    scope = String::from("global");
                    prompt = String::from("> ");
                }
            }
            &"help" => println!("{}", usage),
            &"exit" => exit(0),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
