
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
