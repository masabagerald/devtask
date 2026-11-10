#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Todo,
    InProgress,
    Completed,
    Blocked { blocked_reason: String },
}

impl Status {
    pub fn can_transition_to(&self, next: &Status) -> bool {
        match (self, next) {
            (Status::Todo, Status::InProgress) => true,
            (Status::Todo, Status::Blocked { .. }) => true,
            (Status::InProgress, Status::Completed) => true,
            (Status::InProgress, Status::Blocked { .. }) => true,
            (Status::Blocked { .. }, Status::InProgress) => true,
            _ => false,
        }
    }
    pub fn label(&self) -> &str {
        match self {
            Status::Todo => "Todo",
            Status::InProgress => "In Progress",
            Status::Completed => "Completed",
            Status::Blocked { .. } => "Blocked",
        }
    }
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
