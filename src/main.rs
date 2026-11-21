use devtask::error::TaskError;
use devtask::priority::Priority;
use devtask::status::Status;
use devtask::task_list::TaskList;

use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}


fn run() -> Result<(), TaskError> {
    let mut list = TaskList::new();


    //let _bad = devtask::task::Task::new(99, "   ".into(), Priority::Low)?; 

    list.add("Learn Rust".into(), Priority::High)?;
    list.add("Build a Rust project".into(), Priority::Critical)?;
    list.add("Write docs".into(), Priority::Medium)?;
    println!("Total tasks: {}", list.len());

    // Status changes, including two that fail
    for result in [
        list.start_task(2),
        list.complete_task(2),
        list.block_task(3, "Waiting on API keys".into()),
        list.start_task(99),
    ] {
        if let Err(e) = result {
            println!("Error: {e}");
        }
    }

    println!("In progress: {}", list.count_by_status(&Status::InProgress));

    println!("--- Search: \"rust\" ---");
    for task in list.search("rust") {
        println!("{task}");
    }

    println!("--- By priority ---");
    for task in list.sorted_by_priority() {
        println!("{task}");
    }
   
    Ok(())
}