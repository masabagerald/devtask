use crate::error::TaskError;
use crate::priority::Priority;
use crate::task::Task;

pub struct TaskList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: String, priority: Priority) -> Result<u32, TaskError> {
        let task = Task::new(self.next_id, title, priority)?;
        let id = task.id();
        self.tasks.push(task);
        self.next_id += 1;
        Ok(id)
    }

    pub fn get(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id() == id)
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_assigned_in_order() {
        let mut list = TaskList::new();
        let first = list.add("First".into(), Priority::Low).unwrap();
        let second = list.add("Second".into(), Priority::Low).unwrap();
        assert_eq!(first, 1);
        assert_eq!(second, 2);
    }

    #[test]
    fn get_missing_id_returns_none() {
        // create a list, add one task, then check that list.get(99) is None
    }

    #[test]
    fn blank_title_does_not_use_an_id() {
        // 1. add a blank title "   " and check that it is an error
        // 2. add a valid title and check that its id is 1
    }
}
