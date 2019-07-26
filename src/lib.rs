/// CLI based todo app, for linux, windows and mac
extern crate app_dirs;
extern crate serde_derive;

use app_dirs::*;
use serde_derive::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::fs;
use std::ops::Range;
use std::path::PathBuf;

const APP_INFO: AppInfo = AppInfo {
    name: "todo",
    author: "Tom Wawer",
};
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
}

impl TodoConfig {
    pub fn new() -> TodoConfig {
        TodoConfig {
            data_dir_name: get_app_root(AppDataType::UserConfig, &APP_INFO)
                .expect("App dir not found"),
            data_file_name: DATA_FILE.to_string(),
        }
    }
    // config file in the user app directory
    pub fn load_config(&mut self) {
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
    pub fn remove_data_file(&self) {
        let _ = fs::remove_file(&self.data_dir_name.join(&self.data_file_name));
    }
    pub fn print(&self) {
        println!("{} Todo: ", self.data_file_name);
    }
}

/// Error type for our parsing function
type Result<T> = std::result::Result<T, TodoError>;
#[derive(Debug, Clone)]
pub enum TodoError {
    Add,
    Mark,
    Delete,
    List,
    File,
    Prioriy,
    Undo,
    InvalidCommand,
}
impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "todo parse error")
    }
}
// This is important for other errors to wrap this one.
impl error::Error for TodoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

// main parsing command, takes arguments, skips 0 index
pub fn parse_command(
    conf: &mut TodoConfig,
    data: &mut TodoList,
    arguments: &Vec<String>,
) -> Result<()> {
    let command = arguments[0].to_lowercase();
    let lowercase_command = command.as_str();

    match lowercase_command {
        "g" | "get" | "l" | "list" => Ok(()),
        "a" | "add" => {
            if arguments.len() < 2 {
                return Err(TodoError::Add);
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
            Ok(())
        }
        "d" | "del" => {
            if arguments.len() != 2 {
                // println!("Only one pos argument after del");
                return Err(TodoError::Delete);
            }
            data.make_backup(&conf);
            let item = get_item_set(&arguments[1]);
            match item {
                ReturnItem::IntNum(i) => {
                    data.delete(i);
                    Ok(())
                }
                ReturnItem::IntRange(ir) => {
                    // must be reversed to remove last first
                    for i in ir.rev() {
                        if i < data.list.len() {
                            data.delete(i);
                        }
                    }
                    Ok(())
                }
                ReturnItem::None => {
                    // println!("Nothing deleted check your range");
                    Err(TodoError::Delete)
                }
            }
        }
        "m" | "mark" => {
            if arguments.len() < 2 {
                return Err(TodoError::Mark);
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
                    ReturnItem::None => (),
                }
            }
            Ok(())
        }
        "p" | "pri" => {
            if arguments.len() != 3 {
                return Err(TodoError::Prioriy);
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
            Ok(())
        }
        "u" | "undo" => {
            if arguments.len() != 1 {
                return Err(TodoError::Undo);
            }
            data.read_from_backup(conf);
            Ok(())
        }
        "f" | "file" => {
            if arguments.len() != 2 {
                return Err(TodoError::File);
            }
            data.save(conf);
            conf.data_file_name = arguments[1].to_string();
            conf.save_config();
            data.list = Vec::new();
            data.load(conf);
            Ok(())
        }
        _ => {
            return Err(TodoError::InvalidCommand);
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
