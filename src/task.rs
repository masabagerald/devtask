use std::fmt;

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
    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn priority(&self) -> &Priority {
        &self.priority
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

    pub fn short_title(&self, max_chars: usize) -> &str {
        match self.title.char_indices().nth(max_chars) {
            Some((byte_index, _)) => &self.title[..byte_index],
            None => &self.title,
        }
    }
}

//Display for Task, in task.rs. Produce the same text as summary(), then delete summary() and update the callers. Inside fmt, you can use {} with self.priority and self.status directly, because they implement Display now.

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {} {} (Priority: {}, {})",
            self.id, self.title, self.priority, self.status
        )
    }
}

pub fn longer_title<'a>(task1: &'a Task, task2: &'a Task) -> &'a str {
    if task1.title.len() >= task2.title.len() {
        &task1.title
    } else {
        &task2.title
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
        assert_eq!(task.status(), &Status::Todo);
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

    #[test]
    fn short_title_cuts_ascii() {
        let task = Task::new(1, "Write docs".into(), Priority::Low).unwrap();
        assert_eq!(task.short_title(5), "Write");
    }

    #[test]
    fn short_title_respects_multibyte_chars() {
        let task = Task::new(1, "Café au lait".into(), Priority::Low).unwrap();
        assert_eq!(task.short_title(4), "Café");
    }

    #[test]
    fn short_title_keeps_short_titles_whole() {
        let task = Task::new(1, "Docs".into(), Priority::Low).unwrap();
        assert_eq!(task.short_title(10), "Docs");
    }

    #[test]
    fn longer_title_returns_longer_title() {
        let task1 = Task::new(1, "Short".into(), Priority::Low).unwrap();
        let task2 = Task::new(2, "Longer title".into(), Priority::Low).unwrap();
        assert_eq!(longer_title(&task1, &task2), "Longer title");
    }

   
}
