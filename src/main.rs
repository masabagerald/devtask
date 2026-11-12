use devtask::error::TaskError;

use devtask::priority::Priority;

use devtask::task_list::TaskList;

fn main() -> Result<(), TaskError> {
    let mut list = TaskList::new();

    let id1 = list.add("Learn Rust".into(), Priority::High)?;
    let id2 = list.add("Build a Rust project".into(), Priority::Critical)?;
    let id3 = list.add("Write docs".into(), Priority::Medium)?;

    match list.start_task(2) {
        Ok(()) => println!("Task #2 started"),
        Err(e) => println!("Error: {}", e.message()),
    }

    match list.complete_task(2) {
        Ok(()) => println!("Task #2 completed"),
        Err(e) => println!("Error: {}", e.message()),
    }

    match list.start_task(99) {
        Ok(()) => println!("Task #99 started"),
        Err(e) => println!("Error: {}", e.message()),
    }

    if let Some(task) = list.get(2) {
        println!("{}", task.summary());
    }

    println!("Added tasks {id1}, {id2}, {id3}");
    println!("Total tasks: {}", list.len());

    match list.get(2) {
        Some(task) => println!("Found: {}", task.summary()),
        None => println!("No task with id 2"),
    }

    match list.get(99) {
        Some(task) => println!("Found: {}", task.summary()),
        None => println!("No task with id 99"),
    }

    Ok(())
}
