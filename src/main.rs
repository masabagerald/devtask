#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    priority: u8,
    completed: bool,
}

impl Task {
    fn new(id: u32, title: String, priority: u8) -> Self {
        Task {
            id,
            title,
            priority,
            completed: false,
        }
    }
    fn complete(&mut self) {
        self.completed = true;
    }

    fn summary(&self) -> String {
        let status = if self.completed { "completed" } else { "todo" };

        format!("{} (Priority: {},{})", self.title, self.priority, status)
    }
}

fn main() {
    let task = Task::new(1, "Learn Rust".into(), 3);

    let mut task2 = Task::new(3, "Build a Rust project".into(), 5);

    task2.title = "Build a Rust project with Cargo".into();
    task2.complete();

    let title = String::from("Write docs");

    let task3 = Task::new(4, title.clone(), 2);
    println!("Original title: {title}");

    print_task(&task2);
    print_task(&task2);

    println!("{}", task.summary());

    println!("{}", task2.summary());
    println!("{}", task3.summary());
}

fn print_task(task: &Task) {
    println!("{}", task.summary());
}
