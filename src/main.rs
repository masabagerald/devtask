#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    priority: Priority,
    completed: Status,
}

impl Task {
    fn new(id: u32, title: String, priority: Priority) -> Self {
        Task {
            id,
            title,
            priority,
            completed: Status::Todo,
        }
    }

    fn start(&mut self) {
        self.completed = Status::InProgress;
    }
    fn complete(&mut self) {
        self.completed = Status::Completed;
    }

    fn summary(&self) -> String {
        let status = self.completed.label();

        format!("{} (Priority: {},{})", self.title, self.priority.label(), status)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {

    fn label(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Critical => "Critical",
        }
    }
    
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Status {
    Todo,
    InProgress,
    Completed,
} 

impl Status {
    fn label(&self) -> &str {
        match self {
            Status::Todo => "Todo",
            Status::InProgress => "In Progress",
            Status::Completed => "Completed",
        }
    }
}   

fn main() {
    let task = Task::new(1, "Learn Rust".into(), Priority::High);

    let mut task2 = Task::new(3, "Build a Rust project".into(), Priority::Critical);

    task2.title = "Build a Rust project with Cargo".into();
    task2.complete();

    let title = String::from("Write docs");

    let task3 = Task::new(4, title.clone(), Priority::Medium);
    println!("Original title: {title}");

    print_task(&mut task2);
   

    println!("{}", task.summary());

    println!("{}", task2.summary());
    println!("{}", task3.summary());
}


fn print_task(task: &mut Task) {

    task.complete();
    println!("{}", task.summary());
}
