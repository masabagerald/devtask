use std::fmt;

use crate::status::Status;

#[derive(Debug, PartialEq)]
pub enum TaskError {
    InvalidTransition {
        task_id: u32,
        from: Status,
        to: Status,
    },
    EmptyTitle,
    TaskNotFound(u32),
}

impl TaskError {
    pub fn message(&self) -> String {
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
            TaskError::TaskNotFound(id) => format!("Task with id {} not found", id),
        }
    }
}

//Display for TaskError, in error.rs. Move the text from message() into fmt, then delete message(). Rust will point at every call you need to change, and each e.message() becomes just e inside println!.

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskError::InvalidTransition { task_id, from, to } => write!(
                f,
                "Task #{} cannot transition from {} to {}",
                task_id, from, to
            ),
            TaskError::EmptyTitle => write!(f, "Task title cannot be empty"),
            TaskError::TaskNotFound(id) => write!(f, "Task #{} not found", id),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_displays_readable_text() {
        assert_eq!(TaskError::TaskNotFound(99).to_string(), "Task #99 not found");
    }
}