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
    fn get_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id() == id)
    }

    pub fn start_task(&mut self, id: u32) -> Result<(), TaskError> {
        let task = self.get_mut(id).ok_or(TaskError::TaskNotFound(id))?;
        task.start()
    }

    pub fn complete_task(&mut self, id: u32) -> Result<(), TaskError> {
        let task = self.get_mut(id).ok_or(TaskError::TaskNotFound(id))?;
        task.complete()
    }

    pub fn block_task(&mut self, id: u32, reason: String) -> Result<(), TaskError> {
        let task = self.get_mut(id).ok_or(TaskError::TaskNotFound(id))?;
        task.block(reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::status::Status;

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

    #[test]
    fn starting_missing_task_fails() {
        let mut list = TaskList::new();
        assert_eq!(list.start_task(99), Err(TaskError::TaskNotFound(99)));
    }

    #[test]
    fn start_task_changes_status() {
        let mut list = TaskList::new();
        let id = list.add("Test".into(), Priority::Low).unwrap();
        list.start_task(id).unwrap();

        let task = list.get(id).unwrap();
        assert_eq!(task.status(), &Status::InProgress);
    }
}
