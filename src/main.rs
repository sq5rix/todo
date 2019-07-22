extern crate app_dirs;
extern crate serde_derive;

use app_dirs::*;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

const APP_INFO: AppInfo = AppInfo {
    name: "todo",
    author: "Tom Wawer",
};

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut todo_list = TodoList::new();
    println!("{:?}", todo_list.filename);
    todo_list.load();
    if arguments.len() == 1 {
        todo_list.print();
        print_help();
    }
    todo_list.parse_command(&arguments);
    todo_list.save();
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
    filedir: PathBuf,
    filename: PathBuf,
}

impl TodoList {
    fn new() -> TodoList {
        let dir = get_app_root(AppDataType::UserConfig, &APP_INFO).expect("App dir not found");
        fs::create_dir_all(&dir).expect("Problem creating user data directory");
        TodoList {
            list: Vec::new(),
            filedir: dir,
            filename: PathBuf::from("todo.data"),
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
        println!("Todo:");
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
                    print_help();
                }
                self.add(arguments[2].clone());
                self.print();
            }
            "d" | "del" => {
                if arguments.len() != 3 {
                    print_help();
                }
                self.delete(arguments[2].parse().expect("task number expected"));
                self.print();
            }
            "m" | "mark" => {
                if arguments.len() != 3 {
                    print_help();
                }
                self.mark(arguments[2].parse().expect("task number expected"));
                self.print();
            }

            _ => print_help(),
        }
    }
    fn save(&self) {
        // Convert the TodoList struct to a JSON string.
        let serialized = serde_json::to_string(&self).unwrap();
        fs::write(&self.filename, serialized).expect("Cannot write to file, permissions?");
    }
    fn load(&mut self) {
        // Convert the JSON string back to a TodoList.
        if let Ok(contents) = fs::read_to_string(&self.filename) {
            *self = serde_json::from_str(&contents).unwrap();
        }
    }
}

fn print_help() {
    println!(
        "
    Usage:
        todo add  | a   <name>  # add a todo, if spaces use \"todo today\"
        todo get  | g           # list all items  
        todo list | l           # list all items
        todo mark | m   <num>   # toggle done
        todo del  | d   <num>   # remove todo
        todo file | f   <name>  # specify file name to write list
        todo help               # print help
    "
    );
    ::std::process::exit(0);
}
