Simple command-line todo list, saves todo list in a local file. 
Written in Rust.

    Usage:

        todo file | f   <name>        # specify todo list to use   

        todo read | r   <name>        # read from other todo list into current

        todo add  | a   <name>        # add a todo

        todo get  | g                 # list all items  

        todo list | l                 # list all items

        todo mark | m   <num> [num]* num1..num2  # toggle done

        todo del  | d   <num>         # remove todo

        todo swap | s   <num> <num>   # swap two items

        todo undo | u                 # undo last operation

        todo help                     # print help

    
