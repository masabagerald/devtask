#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    priority: Priority,
    status: Status,
}

impl Task {
    fn new(id: u32, title: String, priority: Priority) -> Self {
        Task {
            id,
            title,
            priority,
            status: Status::Todo,
        }
    }

    fn start(&mut self) {
        self.status = Status::InProgress;
    }
    fn complete(&mut self) {
        self.status = Status::Completed;
    }

    fn block(&mut self, reason: String) {
        self.status = Status::Blocked {
            blocked_reason: reason,
        };
    }

    fn summary(&self) -> String {
        let status_text = match &self.status {
            Status::Blocked { blocked_reason } => format!("Blocked: {}", blocked_reason),
            other => other.label().to_string(),
        };

        format!(
            "id: {} {} (Priority: {},{})",
            self.id,
            self.title,
            self.priority.label(),
            status_text
        )
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

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Todo,
    InProgress,
    Completed,
    Blocked { blocked_reason: String },
}

impl Status {
    fn label(&self) -> &str {
        match self {
            Status::Todo => "Todo",
            Status::InProgress => "In Progress",
            Status::Completed => "Completed",
            Status::Blocked { .. } => "Blocked",
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

    let mut task4 = Task::new(5, "Test the project".into(), Priority::Low);

    task4.start();

    let mut task5 = Task::new(6, "Fix bugs".into(), Priority::High);
    task5.block("Waiting for feedback".into());

    print_task(&mut task2);

    println!("{}", task.summary());

    println!("{}", task2.summary());
    println!("{}", task3.summary());
    println!("{}", task4.summary());
    println!("{}", task5.summary());
}

fn print_task(task: &mut Task) {
    task.start();
    println!("{}", task.summary());
}
