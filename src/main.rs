use std::{io::{self, stdin, Write}, process};
mod todo;
use todo::{add_task, del_task, display_list, get_task, Task};


// Start CLI interface
fn start_cli(){
    println!("Starting ToDo App...");
    println!("'help' for commands to get you started");
    println!("'exit' to exit the program");

    // init todo list
    let mut todo_list: Vec<Task> = Vec::new();

    
    loop{
        // Prompt for input
        println!("\nEnter command: ");
        io::stdout().flush().expect("can't flush to stdout"); // Clear input buffer
        
        // Read command input
        let mut input = String::new();
        stdin().read_line(&mut input).expect("cannot read");
        let args: Vec<&str> = input.split_whitespace().collect();

        parse_args(args, &mut todo_list);
    }
    
}

// Parse CLI input args
fn parse_args(args: Vec<&str>, todo_list: &mut Vec<Task>){
    let command = args[0];

     // Using .get instead of index provides more safety as .get returns
    // None instead of panicking when the index is out of bounds
    match command{
        "add" => {
            if let Some(task_name) = args.get(1){
                let new_task = *task_name;
                add_task(todo_list, new_task);
                display_list(todo_list);
                
            }else{
                println!("Please provide a new name for the task!");
            }
        },
        "del" => {  
            match &args[1].parse::<u64>(){
                Ok(id_no) => {
                    del_task(todo_list, *id_no);
                }

                Err(message) => {
                    println!("{}", message.to_string());
                }
            }
        },
        "edit" => {
            match &args[1].parse::<u64>(){
                Ok(task_id) => {
                    if let Ok(task) = get_task(todo_list, *task_id){
                        // Get new task name
                        if let Some(new_name) = args.get(2){
                            let new_task = *new_name;
                            task.update_task_name(new_task.into());
                            display_list(todo_list);

                        }else{
                            println!("no new task name provided!")
                        }
                    }
                }

                Err(message) => {
                    println!("{}", message.to_string());
                }
            }
        },
        "done" => {
            match &args[1].parse::<u64>() {
                Ok(task_id) => {
                    if let Ok(task) = get_task(todo_list, *task_id){
                        println!("Done task {}", task_id);
                        task.update_status();
                        display_list(todo_list);

                    }else{
                        println!("task id not found in list");
                    }
                }

                Err(message) => {
                    println!("{}", message.to_string());
                }
            }
        },
        "show" => display_list(todo_list),
        "exit" => process::exit(0),
        "help" | _=> display_help(),
    }

}

fn display_help(){
    let help = "
    Query structure:
        [command] [arguments]
    
    Supported Commands:
    
    add - add new task to todo list
        usage: > add task_string
    
    show - show all tasks on todo list
        usage: > show
    
    delete - delete task from todo list
        usage: > del task_id

    done - update task status in todo list
        usage: > done task_id

    edit - edit task name in todo list
        usage: > edit task_id new_task_name 
    
    exit - exit program
        usage: > exit
    
    help - displays help message
        usage: > help ";

    println!("{}", help);
}

fn main() {
    start_cli();
}

