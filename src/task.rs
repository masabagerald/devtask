use crate::error::TaskError;
use crate::priority::Priority;
use crate::status::Status;

#[derive(Debug)]
pub struct Task {
    id: u32,
    title: String,
    priority: Priority,
    status: Status,
}

impl Task {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn rename(&mut self, title: String) -> Result<(), TaskError> {
        if title.trim().is_empty() {
            return Err(TaskError::EmptyTitle);
        }
        self.title = title;
        Ok(())
    }
    pub fn new(id: u32, title: String, priority: Priority) -> Result<Self, TaskError> {
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

    pub fn start(&mut self) -> Result<(), TaskError> {
        self.transition_to(Status::InProgress)
    }

    pub fn complete(&mut self) -> Result<(), TaskError> {
        self.transition_to(Status::Completed)
    }

    pub fn block(&mut self, reason: String) -> Result<(), TaskError> {
        self.transition_to(Status::Blocked {
            blocked_reason: reason,
        })
    }

    pub fn summary(&self) -> String {
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

    pub fn transition_to(&mut self, next: Status) -> Result<(), TaskError> {
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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn rename_rejects_blank_title() {
        let mut task = Task::new(1, "Test".into(), Priority::Low).unwrap();
        assert_eq!(task.rename("  ".into()), Err(TaskError::EmptyTitle));
    }
}
