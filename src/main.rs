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
const CONFIG_FILE: &'static str = "todo.config";
const DATA_FILE: &'static str = "todo.data";

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut config_data = TodoConfig::new();
    let mut todo_list = TodoList::new();

    config_data.load_config();
    todo_list.load(&config_data);

    if arguments.len() == 1 {
        todo_list.print();
        print_help();
    }
    todo_list.parse_command(&arguments);
    todo_list.save(&config_data);
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
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { list: Vec::new() }
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
    fn save(&self, conf: &TodoConfig) {
        // Convert the TodoList struct to a JSON string.
        let todo_data = serde_json::to_string(&self).unwrap();
        let file_name = conf.data_dir_name.join(&conf.data_file_name);
        fs::write(file_name, todo_data).expect("Cannot write to file, permissions?");
    }
    fn load(&mut self, conf: &TodoConfig) {
        // Convert the JSON string back to a TodoList.
        let file_name = conf.data_dir_name.join(&conf.data_file_name);
        if let Ok(todo_data) = fs::read_to_string(file_name) {
            *self = serde_json::from_str(&todo_data).unwrap();
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TodoConfig {
    data_dir_name: PathBuf,
    data_file_name: String,
}

impl TodoConfig {
    fn new() -> TodoConfig {
        TodoConfig {
            data_dir_name: get_app_root(AppDataType::UserConfig, &APP_INFO)
                .expect("App dir not found"),
            data_file_name: DATA_FILE.to_string(),
        }
    }
    // config file in the user app directory
    fn load_config(&mut self) {
        fs::create_dir_all(&self.data_dir_name).expect("Problem creating user data directory");
        let config_file_name = self.data_dir_name.join(CONFIG_FILE);
        if let Ok(contents) = fs::read_to_string(config_file_name) {
            *self = serde_json::from_str(&contents).unwrap();
        } else {
            let serialized = serde_json::to_string(&self).unwrap();
            let file_name = self.data_dir_name.join(CONFIG_FILE);
            fs::write(file_name, serialized).expect("Cannot write to config file, permissions?");
        };
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
