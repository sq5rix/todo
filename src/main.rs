/// CLI based todo app, for linux, windows and mac
use std::env;

use todo::parse_command;
use todo::TodoConfig;
use todo::TodoList;

fn main() {
    let mut arguments: Vec<String> = env::args().collect();
    let mut config_data = TodoConfig::new();
    let mut todo_list = TodoList::new();

    config_data.load_config();
    todo_list.load(&config_data);

    if arguments.len() == 1 {
        config_data.print();
        todo_list.print();
        print_help();
    }

    arguments.remove(0);
    // main parsing command takes config struct and todo list struct
    parse_command(&mut config_data, &mut todo_list, &arguments).unwrap_or_else(|e| match e {
        todo::TodoError::Add => println!("Should be todo add any text, you can use \" < > | : \" "),
        todo::TodoError::Mark => println!("Should be todo mark 3  4..6 etc..."),
        todo::TodoError::Delete => println!("Should be todo del 3 or todo del 3..5"),
        todo::TodoError::List => println!("Use todo list or todo l or todo g or todo get"),
        todo::TodoError::File => println!("Use todo file name - a file with todo items"),
        todo::TodoError::Prioriy => println!("Use todo pri 3 8 from - to"),
        todo::TodoError::Undo => println!("Use todo undo to returne to previous list"),
        todo::TodoError::InvalidCommand => println!("Use correct command"),
    });
    config_data.print();
    todo_list.print();

    if !todo_list.is_empty() {
        todo_list.save(&config_data);
    } else {
        config_data.remove_data_file();
    }
}

// prints help
fn print_help() {
    println!(
        "
    Usage:
        todo file | f   <name>        # specify todo list to use
        todo undo | u                 # undo last operation
        todo add  | a   <name>        # add a todo
        todo get  | g                 # list all items  
        todo list | l                 # list all items
        todo mark | m   [num]* [num1..num2]   # toggle done
        todo del  | d   [num] | [num1..num2]  # remove todo
        todo pri  | p   <num1> <num2> # move from num1 to num2
        todo help                     # print help
    "
    );
    ::std::process::exit(0);
}
