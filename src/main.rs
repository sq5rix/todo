use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let mut todo_list = TodoList::new();

    todo_list.add("one".to_string());
    todo_list.add("two".to_string());

    parse_command(&arguments, &mut todo_list);
}

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
}

fn parse_command(arguments: &Vec<String>, todo_list: &mut TodoList) {
    let command = arguments[1].as_str();

    match command {
        "g" | "get" | "l" | "list" => {
            todo_list.print();
        }
        "a" | "add" => {
            if arguments.len() != 3 {
                print_help();
            }
            todo_list.add(arguments[2].clone());
            todo_list.print();
        }
        "d" | "del" => {
            if arguments.len() != 3 {
                print_help();
            }
            todo_list.add(arguments[2].clone());
            todo_list.print();
        }
        "m" | "mark" => {
            if arguments.len() != 3 {
                print_help();
            }
            todo_list.mark(arguments[2].parse().expect("task number expected"));
            todo_list.print();
        }
        _ => print_help(),
    }
}

fn print_help() {
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
