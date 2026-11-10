mod error;
mod priority;
mod status;
mod task;
use error::TaskError;
use priority::Priority;

use task::Task;

fn main() -> Result<(), TaskError> {
    let task = Task::new(1, "Learn Rust".into(), Priority::High)?;

    let mut task2 = Task::new(3, "Build a Rust project".into(), Priority::Critical)?;

    task2.rename("Build a Rust project with Cargo".into())?;

    match task2.start() {
        Ok(()) => println!("Task #{} started", task2.id()),
        Err(e) => println!("Error: {}", e.message()),
    }

    match task2.complete() {
        Ok(()) => println!("Task #{} completed", task2.id()),
        Err(e) => println!("Error: {}", e.message()),
    }

    // Deliberately illegal: a completed task cannot restart
    match task2.start() {
        Ok(()) => println!("Task #{} restarted", task2.id()),
        Err(e) => println!("Error: {}", e.message()),
    }

    let title = String::from("Write docs");

    let task3 = Task::new(4, title.clone(), Priority::Medium)?;
    println!("Original title: {title}");

    let mut task4 = Task::new(5, "Test the project".into(), Priority::Low)?;

    match task4.start() {
        Ok(()) => println!("Task #{} started", task4.id()),
        Err(e) => println!("Error: {}", e.message()),
    }

    let mut task5 = Task::new(6, "Fix bugs".into(), Priority::High)?;

    match task5.block("Waiting for feedback".into()) {
        Ok(()) => println!("Task #{} blocked", task5.id()),
        Err(e) => println!("Error: {}", e.message()),
    }

    print_task(&task2);

    //println!("{}", task.summary());

    println!("{}", task2.summary());
    println!("{}", task3.summary());
    println!("{}", task4.summary());
    println!("{}", task5.summary());

    println!("{}", task.summary());

    Ok(())
}

fn print_task(task: &Task) {
    println!("{}", task.summary());
}
