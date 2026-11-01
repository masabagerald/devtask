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
    fn can_transition_to(&self, next: &Status) -> bool {
        match (self, next) {
            (Status::Todo, Status::InProgress) => true,
            (Status::Todo, Status::Blocked { .. }) => true,
            (Status::InProgress, Status::Completed) => true,
            (Status::InProgress, Status::Blocked { .. }) => true,
            (Status::Blocked { .. }, Status::InProgress) => true,
            _ => false,
        }
    }
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

    println!("{}", Status::Todo.can_transition_to(&Status::InProgress)); // expect true
    println!(
        "{}",
        Status::Completed.can_transition_to(&Status::InProgress)
    ); // expect false
    println!(
        "{}",
        Status::InProgress.can_transition_to(&Status::Completed)
    ); // expect true
    println!(
        "{}",
        Status::InProgress.can_transition_to(&Status::Blocked {
            blocked_reason: "Waiting for feedback".into()
        })
    ); // expect true
    println!("{}", Status::Todo.can_transition_to(&Status::Todo)); // expect true

    println!(
        "{}",
        Status::Blocked {
            blocked_reason: "Waiting for feedback".into()
        }
        .can_transition_to(&Status::InProgress)
    ); // expect true

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo_can_move_to_in_progress() {
        assert!(Status::Todo.can_transition_to(&Status::InProgress));
    }

    #[test]
    fn completed_is_final() {
        assert!(!Status::Completed.can_transition_to(&Status::InProgress));
    }

    #[test]
    fn in_progress_can_move_to_completed() {
        assert!(Status::InProgress.can_transition_to(&Status::Completed));
    }

    #[test]
    fn blocked_can_move_to_in_progress() {
        assert!(
            Status::Blocked {
                blocked_reason: "Waiting for feedback".into()
            }
            .can_transition_to(&Status::InProgress)
        );
    }

    #[test]
    fn todo_cannot_move_to_completed() {
        assert!(!Status::Todo.can_transition_to(&Status::Completed));
    }
    #[test]
    fn todo_can_move_to_blocked() {
        assert!(Status::Todo.can_transition_to(&Status::Blocked {
            blocked_reason: "Waiting for feedback".into()
        }));
    }

    #[test]
    fn blocked_cannot_move_to_completed() {
        assert!(
            !Status::Blocked {
                blocked_reason: "Waiting for feedback".into()
            }
            .can_transition_to(&Status::Completed)
        );
    }

    #[test]
    fn same_status_is_rejected() {
        assert!(!Status::Todo.can_transition_to(&Status::Todo));
    }
}
