#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}
impl Priority {
    pub fn label(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Critical => "Critical",
        }
    }
}
