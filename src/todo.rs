use std::sync::atomic::{self, AtomicU64};

pub struct Task {
    task: String,
    done_status: bool,
    id: u64,
}

// Feels like a class, impl (implement?) allows you to add functions that only apply to instances of the struct
impl Task {
    pub fn update_status(&mut self) {
        self.done_status = true;
    }

    pub fn update_task_name(&mut self, new_name: String) {
        self.task = new_name;
    }
}


// Return task that is chosen
pub fn get_task(todo_list: &mut Vec<Task>, task_id: u64) -> Result<&mut Task, &str>{
    for task in todo_list{
        if task_id == task.id{
            return Ok(task);
        }else{
            continue;
        }
    };
    return  Err("Task not found in to do list");
}    


static UNIQUE_ID: AtomicU64 = AtomicU64::new(1);

// Add task to todo list
pub fn add_task(todo_list: &mut Vec<Task>, task_string: &str) {
    let task_id = UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst);

    let task: Task = Task {
        task: task_string.into(),
        done_status: false,
        id: task_id,
    };

    todo_list.push(task);
    println!("'{}' added to to do list!", task_string);
}

// Delete task from list
pub fn del_task(todo_list: &mut Vec<Task>, task_id: u64) {
    todo_list.retain(|task| task.id != task_id);
    println!("Task ID {} deleted from to do list!", task_id);
}

// Display todo List
pub fn display_list(todo_list: &mut Vec<Task>) {
    if todo_list.len() < 1 {
        println!("Your to do list is empty!");
        return;
    }
    println!();
    
    for item in todo_list {
        println!(
            "Task ID: {}, Task Name: {}, Done: {}",
            item.id, item.task, item.done_status
        )
    }
}
