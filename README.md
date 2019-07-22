Simple command-line todo list, saves todo list in a local file. 
Written in Rust.
Use as-is. Todo:
0. [ ] - change data file name for multiple lists
1. [ ] - remove data file after deleting last item
2. [ ] - implement date & time
3. [ ] - implement sorting/moving items

    Usage:
        todo add  | a   <name>  # add a todo, if spaces use "todo today"
        todo get  | g           # list all items  
        todo list | l           # list all items
        todo mark | m   <num>   # toggle done
        todo del  | d   <num>   # remove todo
        todo file | f   <name>  # specify file name to write list
        todo help               # print help
    
