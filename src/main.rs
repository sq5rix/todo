// CLI based todo app, for linux, windows and mac

extern crate app_dirs;
extern crate serde_derive;

use app_dirs::*;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

use todo::get_range;

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
        config_data.print();
        todo_list.print();
        print_help();
    }

    // main parsing command takes config struct and todo list struct
    parse_command(&mut config_data, &mut todo_list, &arguments);

    if !todo_list.is_empty() {
        todo_list.save(&config_data);
    } else {
        config_data.remove_data_file();
    }
}

// main todoitem structure, keeps a single todo
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

// Vec of TodoItems
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
        for (idx, item) in self.list.iter().enumerate() {
            println!("{}. [{}] - {}", idx, item.completed, item.item);
        }
    }
    fn is_empty(&self) -> bool {
        self.list.is_empty()
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

// config file struct
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
            self.save_config();
        };
    }
    fn save_config(&mut self) {
        let serialized = serde_json::to_string(&self).unwrap();
        let file_name = self.data_dir_name.join(CONFIG_FILE);
        fs::write(file_name, serialized).expect("Cannot write to config file, permissions?");
    }
    fn remove_data_file(&self) {
        let _ = fs::remove_file(&self.data_dir_name.join(&self.data_file_name));
    }
    fn print(&self) {
        println!("{} Todo: ", self.data_file_name);
    }
}

// main parsing command, takes arguments, skips 0 index
fn parse_command(conf: &mut TodoConfig, data: &mut TodoList, arguments: &Vec<String>) {
    let command = arguments[1].as_str();

    match command {
        "g" | "get" | "l" | "list" => {
            conf.print();
            data.print();
        }
        "a" | "add" => {
            if arguments.len() < 3 {
                print_help();
            }
            let mut todo_item = String::new();
            let mut a = 2;
            while a < arguments.len() {
                todo_item.push_str(&arguments[a]);
                todo_item.push(' ');
                a += 1;
            }
            data.add(todo_item);
            conf.print();
            data.print();
        }
        "d" | "del" => {
            if arguments.len() != 3 {
                print_help();
            }
            data.delete(arguments[2].parse().expect("task number expected"));
            conf.print();
            data.print();
        }
        "m" | "mark" => {
            if arguments.len() < 3 {
                print_help();
            }
            let nums = &arguments[2..];
            for idx in nums {
                let i = idx.parse();
                match i {
                    Ok(_) => {
                        data.mark(i.unwrap());
                    }
                    Err(_) => {
                        if let Some(range) = get_range(idx) {
                            for index in range {
                                data.mark(index);
                            }
                        } else {
                        }
                    }
                }
            }
            conf.print();
            data.print();
        }
        "s" | "swap" => {
            if arguments.len() != 4 {
                print_help();
            }
            let ind1: usize = arguments[2].parse().expect("task 1 number expected");
            let ind2: usize = arguments[3].parse().expect("task 2 number expected");
            data.list.swap(ind1, ind2);
            conf.print();
            data.print();
        }
        "f" | "file" => {
            if arguments.len() != 3 {
                print_help();
            }
            data.save(conf);
            conf.data_file_name = arguments[2].clone();
            conf.save_config();
            data.list = Vec::new();
            data.load(conf);
            conf.print();
            data.print();
        }
        _ => {
            conf.print();
            data.print();
            print_help();
        }
    }
}

// prints help
fn print_help() {
    println!(
        "
    Usage:
        todo file | f   <name>        # specify todo list to use   
        todo add  | a   <name>        # add a todo
        todo get  | g                 # list all items  
        todo list | l                 # list all items
        todo mark | m   <num> [num]* num1..num2  # toggle done
        todo del  | d   <num>         # remove todo
        todo swap | s   <num> <num>   # swap two items
        todo help                     # print help
    "
    );
    ::std::process::exit(0);
}
