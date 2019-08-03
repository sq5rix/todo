Simple command-line todo list, saves todo list in a local file. 
Written in Rust.

Look at screenshots in wiki to see todo in action

    Usage:

        todo file | f   <name>        # load todo list to use   

        todo list | l                 # list all todo lists

        todo read | r   <name>        # read from other todo list into current

        todo add  | a   <name>        # add a todo

        todo get  | g                 # list all items  

        todo mark | m   <num> [num]* num1..num2  # toggle done

        todo del  | d   <num>         # remove todo

        todo swap | s   <num> <num>   # swap two items

        todo undo | u                 # undo last operation

        todo help                     # print help

    
