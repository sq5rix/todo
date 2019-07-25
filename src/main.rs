/// CLI based todo app, for linux, windows and mac
use std::env;

use todo::TodoConfig;
use todo::TodoList;
use todo::parse_command;


fn main() {
    let mut arguments: Vec<String> = env::args().collect();
    let mut config_data = TodoConfig::new();
    let mut todo_list = TodoList::new();

    config_data.load_config();
    todo_list.load(&config_data);

    if arguments.len() == 1 {
        config_data.print();
        todo_list.print();
        todo::print_help();
    }

    arguments.remove(0);
    // main parsing command takes config struct and todo list struct
    parse_command(&mut config_data, &mut todo_list, &arguments);
    config_data.print();
    todo_list.print();

    if !todo_list.is_empty() {
        todo_list.save(&config_data);
    } else {
        config_data.remove_data_file();
    }
}
