Simple command-line todo list, saves todo list in a local file. 
Written in Rust.
Use as-is. 

Todo:

1. [x] - remove data file after deleting last item
2. [ ] - implement date & time
3. [x] - implement sorting/moving items

    Usage:
    
        todo add  | a any string # add a todo

        todo get  | g            # list all items  

        todo list | l            # list all items

        todo mark | m   <num>+   # toggle done

        todo del  | d   <num>    # remove todo

        todo swap | s <num> <num>#swap two items

        todo file | f   <name>   # specify todo list name 

        todo help                # print help

    
