use crate::status::Status;

#[derive(Debug, PartialEq)]
pub enum TaskError {
    InvalidTransition {
        task_id: u32,
        from: Status,
        to: Status,
    },
    EmptyTitle,
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
        }
    }
}
