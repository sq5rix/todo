extern crate serde;
extern crate serde_derive;
// extern crate serde_json;

// use serde_json;
use serde_derive::{Deserialize, Serialize};
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let mut todo_list = TodoList::new();

    todo_list.add("one".to_string());
    todo_list.add("two".to_string());

    todo_list.parse_command(&arguments);
}

#[derive(Serialize, Deserialize)]
struct TodoItem {
    item: String,
    completed: char,
}

impl TodoItem {
    fn new(item: String) -> TodoItem {
        return TodoItem {
            item: item,
            completed: ' ',
        };
    }
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    list: Vec<TodoItem>,
    filename: String,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            list: Vec::new(),
            filename: ".todo".to_string(),
        }
    }
    fn add(&mut self, name: String) {
        self.list.push(TodoItem::new(name));
    }
    fn delete(&mut self, pos: usize) {
        self.list.remove(pos);
    }
    fn mark(&mut self, pos: usize) {
        let mark = self.list[pos].completed;
        if mark == 'x' {
            self.list[pos].completed = ' ';
        } else {
            self.list[pos].completed = 'x';
        }
    }
    fn print(&self) {
        for (idx, item) in self.list.iter().enumerate() {
            println!("{}. [{}] - {}", idx, item.completed, item.item);
        }
    }

    fn parse_command(&mut self, arguments: &Vec<String>) {
        let command = arguments[1].as_str();

        match command {
            "g" | "get" | "l" | "list" => {
                self.print();
            }
            "a" | "add" => {
                if arguments.len() != 3 {
                    self.print_help();
                }
                self.add(arguments[2].clone());
                self.print();
            }
            "d" | "del" => {
                if arguments.len() != 3 {
                    self.print_help();
                }
                self.delete(arguments[2].parse().expect("task number expected"));
                self.print();
            }
            "m" | "mark" => {
                if arguments.len() != 3 {
                    self.print_help();
                }
                self.mark(arguments[2].parse().expect("task number expected"));
                self.print();
            }
            _ => self.print_help(),
        }
    }
    fn save(&self) {}
    fn load(&mut self) {}

    fn print_help(&self) {
        println!(
            "
        Usage:
            todo add | a    <name>  # add a todo
            todo get | g            # list all items  
            todo list | l           # list all items
            todo mark | m   <num>   # toggle done
            todo del | d    <num>   # remove todo
            todo help               # print help
        "
        );
        ::std::process::exit(0);
    }
}
