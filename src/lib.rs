/// CLI based todo app, for linux, windows and mac
extern crate app_dirs;
extern crate serde_derive;

use app_dirs::*;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::ops::Range;
use std::path::PathBuf;

const APP_INFO: AppInfo = AppInfo {
    name: "todo",
    author: "Tom Wawer",
};
/// Command enum Error type for our parsing function
/// For now, it returns only command name
type TodoResult = std::result::Result<TodoParseReturn, TodoParseReturn>;
#[derive(Debug, Clone)]
pub enum TodoParseReturn {
    Add,
    Mark,
    Delete,
    List,
    Get,
    File,
    Priority,
    ReadFile,
    Undo,
    InvalidCommand,
}

const CONFIG_FILE: &'static str = "todo.config";
const DATA_FILE: &'static str = "todo.data";

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
    fn clone(self: &TodoItem) -> TodoItem {
        TodoItem {
            item: self.item.clone(),
            completed: self.completed,
        }
    }
}

// Vec of TodoItems
#[derive(Serialize, Deserialize)]
pub struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
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
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
    fn make_backup(&self, conf: &TodoConfig) {
        // Convert the TodoList struct to a JSON string.
        let todo_data = serde_json::to_string(&self).unwrap();
        let backup_file = conf.data_file_name.clone() + ".bk";
        let file_name = conf.data_dir_name.join(backup_file);
        fs::write(file_name, todo_data).expect("Cannot write to backup file, permissions?");
    }
    fn read_from_backup(&mut self, conf: &TodoConfig) {
        let backup_file: String = conf.data_file_name.clone() + ".bk";
        let file_name = conf.data_dir_name.join(backup_file);
        if let Ok(todo_data) = fs::read_to_string(file_name) {
            *self = serde_json::from_str(&todo_data).unwrap();
        }
    }
    pub fn save(&self, conf: &TodoConfig) {
        // Convert the TodoList struct to a JSON string.
        let todo_data = serde_json::to_string(&self).unwrap();
        let file_name = conf.data_dir_name.join(&conf.data_file_name);
        fs::write(file_name, todo_data).expect("Cannot write to file, permissions?");
    }
    pub fn load_other_file(&mut self, conf: &TodoConfig, other_file: String) -> TodoResult {
        // Convert the JSON string back to a TodoList.
        let file_name = conf.data_dir_name.join(other_file);
        if let Ok(todo_data) = fs::read_to_string(file_name) {
            if let Ok(mut new_data) = serde_json::from_str::<TodoList>(&todo_data) {
                self.list.append(&mut new_data.list);
            } else {
                return Err(TodoParseReturn::ReadFile);
            }
        }
        Ok(TodoParseReturn::ReadFile)
    }
    pub fn load(&mut self, conf: &TodoConfig) {
        // Convert the JSON string back to a TodoList.
        let file_name = conf.data_dir_name.join(&conf.data_file_name);
        if let Ok(todo_data) = fs::read_to_string(file_name) {
            *self = serde_json::from_str(&todo_data).unwrap();
        }
    }
    pub fn print(&self) {
        let mut prefix = "";
        for (idx, item) in self.list.iter().enumerate() {
            if idx < 1000 {
                prefix = ""
            }
            if idx < 100 {
                prefix = " "
            }
            if idx < 10 {
                prefix = "  "
            }
            println!("{}{}. [{}] - {}", prefix, idx, item.completed, item.item);
        }
    }
}

// config file struct
#[derive(Serialize, Deserialize)]
pub struct TodoConfig {
    data_dir_name: PathBuf,
    data_file_name: String,
    data_list: Vec<String>,
}

impl TodoConfig {
    pub fn new() -> TodoConfig {
        TodoConfig {
            data_dir_name: get_app_root(AppDataType::UserConfig, &APP_INFO)
                .expect("App dir not found"),
            data_file_name: DATA_FILE.to_string(),
            data_list: Vec::new(),
        }
    }
    // config file in the user app directory
    pub fn load_config(&mut self) {
        fs::create_dir_all(&self.data_dir_name).expect("Problem creating user data directory");
        let config_file_name = self.data_dir_name.join(CONFIG_FILE);
        if let Ok(contents) = fs::read_to_string(config_file_name) {
            if let Ok(new_data) = serde_json::from_str::<TodoConfig>(&contents) {
                *self = new_data;
            } else {
                println!("saved config - error in load config");
                self.save_config();
            }
        // *self = serde_json::from_str(&contents).unwrap();
        } else {
            self.save_config();
        };
    }
    fn save_config(&mut self) {
        let serialized = serde_json::to_string(&self).unwrap();
        let file_name = self.data_dir_name.join(CONFIG_FILE);
        fs::write(file_name, serialized).expect("Cannot write to config file, permissions?");
    }
    pub fn remove_data_file(&mut self) {
        let file_name = &self.data_file_name.clone();
        self.remove_todo_file(file_name);
        let _ = fs::remove_file(&self.data_dir_name.join(file_name));
        self.save_config();
    }
    pub fn add_todo_file(&mut self, v: String) {
        &self.data_list.retain(|i| i != &v);
        &self.data_list.push(v);
    }
    pub fn remove_todo_file(&mut self, v: &String) {
        &self.data_list.retain(|i| i != v);
    }
    pub fn print(&self) {
        println!("{} Todo: ", self.data_file_name);
    }
    pub fn print_list(&self) {
        println!("Config dir: {:?}", &self.data_dir_name);
        println!("Todo lists:");
        for i in &self.data_list {
            println!("{}", i);
        }
    }
}

// main parsing command, takes arguments, skips 0 index
pub fn parse_command(
    conf: &mut TodoConfig,
    data: &mut TodoList,
    arguments: &Vec<String>,
) -> TodoResult {
    let command = arguments[0].to_lowercase();
    let lowercase_command = command.as_str();

    match lowercase_command {
        "g" | "get" => Ok(TodoParseReturn::Get),
        "l" | "list" => Ok(TodoParseReturn::List),
        "a" | "add" => {
            if arguments.len() < 2 {
                return Err(TodoParseReturn::Add);
            }
            let mut todo_item = String::new();
            let mut a = 1;
            while a < arguments.len() {
                todo_item.push_str(&arguments[a]);
                todo_item.push(' ');
                a += 1;
            }
            data.add(todo_item);
            data.make_backup(&conf);
            Ok(TodoParseReturn::Add)
        }
        "d" | "del" => {
            if arguments.len() != 2 {
                return Err(TodoParseReturn::Delete);
            }
            data.make_backup(&conf);
            let item = get_item_set(&arguments[1]);
            match item {
                ReturnItem::IntNum(i) => {
                    data.delete(i);
                }
                ReturnItem::IntRange(ir) => {
                    // must be reversed to remove last first
                    for i in ir.rev() {
                        if i < data.list.len() {
                            data.delete(i);
                        }
                    }
                }
                ReturnItem::None => {
                    return Err(TodoParseReturn::Delete);
                }
            }
            Ok(TodoParseReturn::Delete)
        }
        "m" | "mark" => {
            if arguments.len() < 2 {
                return Err(TodoParseReturn::Mark);
            }
            let nums = &arguments[1..];
            for idx in nums {
                let item = get_item_set(idx);
                match item {
                    ReturnItem::IntNum(i) => {
                        data.mark(i);
                    }
                    ReturnItem::IntRange(ir) => {
                        for i in ir {
                            if i < data.list.len() {
                                data.mark(i);
                            }
                        }
                    }
                    ReturnItem::None => {
                        return Err(TodoParseReturn::Mark);
                    }
                }
            }
            Ok(TodoParseReturn::Mark)
        }
        "p" | "pri" => {
            if arguments.len() != 3 {
                return Err(TodoParseReturn::Priority);
            }
            let pos: usize = arguments[1].parse().expect("task 1 number expected");
            let goto: usize = arguments[2].parse().expect("task 2 number expected");
            if pos > goto {
                data.list.insert(goto, data.list[pos].clone());
                data.list.remove(pos + 1);
            } else if pos < goto {
                data.list.insert(goto + 1, data.list[pos].clone());
                data.list.remove(pos);
            }
            Ok(TodoParseReturn::Priority)
        }
        "u" | "undo" => {
            if arguments.len() != 1 {
                return Err(TodoParseReturn::Undo);
            }
            data.read_from_backup(conf);
            Ok(TodoParseReturn::Undo)
        }
        "f" | "file" => {
            if arguments.len() != 2 {
                return Err(TodoParseReturn::File);
            }
            data.save(conf);
            conf.data_file_name = arguments[1].to_string();
            conf.add_todo_file(conf.data_file_name.clone());
            conf.save_config();
            data.list = Vec::new();
            data.load(conf);
            Ok(TodoParseReturn::File)
        }
        "r" | "read" => {
            if arguments.len() != 2 {
                return Err(TodoParseReturn::ReadFile);
            }
            return data.load_other_file(conf, arguments[1].to_string());
        }
        _ => {
            return Err(TodoParseReturn::InvalidCommand);
        }
    }
}

/// help parse lib for todo app

/// emun holding returning value for parsing function
/// It return an usize integer or a range
#[derive(Debug, PartialEq)]
pub enum ReturnItem {
    IntNum(usize),
    IntRange(Range<usize>),
    None,
}

/// get item set - an usize integer or a range of item
pub fn get_item_set(s: &str) -> ReturnItem {
    let mut first = String::new();
    let mut second = String::new();
    let mut first_filled = false;
    let mut iter = s.chars();
    while let Some(c) = iter.next() {
        // for c in s.chars() { - cant use nex in for loop!!!
        // possibles are digit or . or - all other are errors
        if c.is_digit(10) && !first_filled {
            first.push(c);
        } else if c.is_digit(10) && first_filled {
            second.push(c);
        } else if !c.is_digit(10) && !first_filled {
            if c == '-' {
                first_filled = true;
            } else if c == '.' {
                if iter.next() == Some('.') {
                    first_filled = true;
                } else {
                    return ReturnItem::None;
                }
            } else {
                return ReturnItem::None;
            }
        } else {
            return ReturnItem::None;
        }
    }
    // println!("Got: r1: {} r2: {}", first, second);
    if let Ok(r1) = first.parse() {
        if let Ok(r2) = second.parse() {
            if r1 < r2 {
                // r2 + 1 to get consistent with human
                return ReturnItem::IntRange(r1..r2 + 1);
            } else {
                return ReturnItem::None;
            }
        } else {
            return ReturnItem::IntNum(r1);
        };
    } else {
        return ReturnItem::None;
    };
}

pub fn todo_error_display(e: TodoParseReturn) {
    match e {
        TodoParseReturn::Add => eprintln!("Use todo add any text, you can use \" < > | : \" "),
        TodoParseReturn::Mark => eprintln!("Use todo mark 3 4..6 etc..."),
        TodoParseReturn::Delete => eprintln!("Use todo del 3 or todo del 3..5"),
        TodoParseReturn::List => eprintln!("Use todo list or todo l "),
        TodoParseReturn::Get => eprintln!("Use todo get or todo g "),
        TodoParseReturn::ReadFile => eprintln!("Use todo file name other than current"),
        TodoParseReturn::File => eprintln!("Use todo file name - a file with todo items"),
        TodoParseReturn::Priority => eprintln!("Use todo pri 3 8 from - to"),
        TodoParseReturn::Undo => eprintln!("Use todo undo to return to previous list"),
        TodoParseReturn::InvalidCommand => eprintln!("Use correct command"),
    }
}

pub fn command_match(c: TodoParseReturn, conf: &TodoConfig, data: &TodoList) {
    match c {
        TodoParseReturn::InvalidCommand => {
            print_help();
        }
        TodoParseReturn::List => conf.print_list(),
        _ => {
            conf.print();
            data.print();
        }
    }
}

/// prints todo parser help
pub fn print_help() {
    println!(
        "
    Usage:
        todo list | l                 # list all todo lists in config directory
        todo file | f   <name>        # load todo list to use
        todo read | r   <name>        # read from other todo list into current
        todo undo | u                 # undo last operation
        todo add  | a   <name>        # add a todo
        todo get  | g                 # list all items  
        todo mark | m   [num]* [num1..num2]   # toggle done
        todo del  | d   [num] | [num1..num2]  # remove todo
        todo pri  | p   <num1> <num2> # move from num1 to num2
        todo help                     # print help
    "
    );
    ::std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::get_item_set;
    use super::ReturnItem;
    #[test]
    fn valid_range_tests() {
        let valid_data = vec!["5..10", "5-10"];
        for test in valid_data {
            assert_eq!(
                get_item_set(&test),
                ReturnItem::IntRange(5..11),
                "we are testing {} as {:?}",
                test,
                ReturnItem::IntRange(5..11)
            );
        }
    }
    #[test]
    fn valid_single_tests() {
        let valid_data = vec!["5"];
        for test in valid_data {
            assert_eq!(
                get_item_set(&test),
                ReturnItem::IntNum(5),
                "we are testing {} as {:?}",
                test,
                ReturnItem::IntNum(5)
            );
        }
    }
    #[test]
    fn invalid_tests() {
        let invalid_data = vec![
            "5x10",
            "[5-10ccc]",
            "[5..10ccc]",
            "5x",
            "x5",
            "61-6a",
            "aaa5",
            "61-6",
            "xxx",
            "61-6",
        ];
        for test in invalid_data {
            assert_eq!(
                get_item_set(&test),
                ReturnItem::None,
                "we are testing {} as None",
                test
            );
        }
    }
}
