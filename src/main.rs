mod priority;
mod status;
use priority::Priority;
use status::Status;

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    priority: Priority,
    status: Status,
}

impl Task {
    fn new(id: u32, title: String, priority: Priority) -> Result<Self, TaskError> {
        if title.trim().is_empty() {
            return Err(TaskError::EmptyTitle);
        }

        Ok(Task {
            id,
            title,
            priority,
            status: Status::Todo,
        })
    }

    fn start(&mut self) -> Result<(), TaskError> {
        self.transition_to(Status::InProgress)
    }

    fn complete(&mut self) -> Result<(), TaskError> {
        self.transition_to(Status::Completed)
    }

    fn block(&mut self, reason: String) -> Result<(), TaskError> {
        self.transition_to(Status::Blocked {
            blocked_reason: reason,
        })
    }

    fn summary(&self) -> String {
        let status_text = match &self.status {
            Status::Blocked { blocked_reason } => format!("Blocked: {}", blocked_reason),
            other => other.label().to_string(),
        };

        format!(
            "id: {} {} (Priority: {}, {})",
            self.id,
            self.title,
            self.priority.label(),
            status_text
        )
    }

    fn transition_to(&mut self, next: Status) -> Result<(), TaskError> {
        if !self.status.can_transition_to(&next) {
            return Err(TaskError::InvalidTransition {
                task_id: self.id,
                from: self.status.clone(),
                to: next,
            });
        }
        self.status = next;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum TaskError {
    InvalidTransition {
        task_id: u32,
        from: Status,
        to: Status,
    },
    EmptyTitle,
}

impl TaskError {
    fn message(&self) -> String {
        match self {
            TaskError::InvalidTransition { task_id, from, to } => {
                format!(
                    "Task #{} cannot transition from {} to {}",
                    task_id,
                    from.label(),
                    to.label()
                )
            }
            TaskError::EmptyTitle => "Task title cannot be empty".into(),
        }
    }
}

fn main() -> Result<(), TaskError> {
    let task = Task::new(1, "Learn Rust".into(), Priority::High)?;

    let mut task2 = Task::new(3, "Build a Rust project".into(), Priority::Critical)?;

    task2.title = "Build a Rust project with Cargo".into();

    match task2.start() {
        Ok(()) => println!("Task #{} started", task2.id),
        Err(e) => println!("Error: {}", e.message()),
    }

    match task2.complete() {
        Ok(()) => println!("Task #{} completed", task2.id),
        Err(e) => println!("Error: {}", e.message()),
    }

    // Deliberately illegal: a completed task cannot restart
    match task2.start() {
        Ok(()) => println!("Task #{} restarted", task2.id),
        Err(e) => println!("Error: {}", e.message()),
    }

    let title = String::from("Write docs");

    let task3 = Task::new(4, title.clone(), Priority::Medium)?;
    println!("Original title: {title}");

    let mut task4 = Task::new(5, "Test the project".into(), Priority::Low)?;

    match task4.start() {
        Ok(()) => println!("Task #{} started", task4.id),
        Err(e) => println!("Error: {}", e.message()),
    }

    let mut task5 = Task::new(6, "Fix bugs".into(), Priority::High)?;

    match task5.block("Waiting for feedback".into()) {
        Ok(()) => println!("Task #{} blocked", task5.id),
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

    /*     fn cannot_complete_a_todo_task() {
        let mut task = Task::new(1, "Test".into(), Priority::Low);
        assert!(task.complete().is_err());
    } */
    #[test]
    fn failed_transition_leaves_status_unchanged() {
        let mut task = Task::new(1, "Test".into(), Priority::Low).unwrap();
        let _ = task.complete();
        assert_eq!(task.status, Status::Todo);
    }

    #[test]
    fn cannot_complete_a_todo_task() {
        let mut task = Task::new(1, "Test".into(), Priority::Low).unwrap();
        assert_eq!(
            task.complete(),
            Err(TaskError::InvalidTransition {
                task_id: 1,
                from: Status::Todo,
                to: Status::Completed,
            })
        );
    }

    #[test]
    fn blank_title_is_rejected() {
        let result = Task::new(1, "   ".into(), Priority::Low);
        assert_eq!(result.unwrap_err(), TaskError::EmptyTitle);
    }
}
