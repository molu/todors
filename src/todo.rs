use phf::phf_map;

pub static PRIORITY_MAP: phf::Map<&'static str, Priority> = phf_map! {
    "P1" => Priority::High,
    "P2" => Priority::Medium,
    "P3" => Priority::Low,
};

#[derive(Debug, PartialEq)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
    pub due_date: u32,
}

impl Todo {
    pub fn new(title: String, description: String, priority: Priority, due_date: u32) -> Todo {
        Todo {
            title: title,
            description: description,
            status: Status::NotStarted,
            priority: priority,
            due_date: due_date,
        }
    }

    pub fn start(&mut self) {
        self.status = Status::InProgress;
    }

    pub fn complete(&mut self) {
        if self.status != Status::InProgress {
            println!("Cannot complete todo that has not been started yet")
        } else {
            println!("Todo {} completed", self.title);
            self.status = Status::Completed;
        }
    }

    pub fn update(
        &mut self,
        title: String,
        description: String,
        priority: Priority,
        due_date: u32,
    ) {
        if title.trim() != "" {
            self.title = title.trim().to_string();
        } else {
            println!("Title not updated ({})", title.trim());
        }

        if description.trim() != "" {
            self.description = description.trim().to_string();
        } else {
            println!("Description not updated ({})", description.trim());
        }

        self.priority = priority;

        if due_date > 0 {
            self.due_date = due_date;
        } else {
            println!("Due date not updated ({})", due_date);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_todo() {
        let todo = Todo::new(
            "Test title".to_string(),
            "Test description".to_string(),
            Priority::High,
            1695844953,
        );
        assert_eq!(todo.title, "Test title");
        assert_eq!(todo.description, "Test description");
        assert_eq!(todo.priority, Priority::High);
        assert_eq!(todo.due_date, 1695844953);
    }

    #[test]
    fn test_start_todo() {
        let mut todo = Todo::new(
            "Test title".to_string(),
            "Test description".to_string(),
            Priority::High,
            1695844953,
        );
        assert_eq!(todo.status, Status::NotStarted);
        todo.start();
        assert_eq!(todo.status, Status::InProgress);
    }

    #[test]
    fn test_complete_todo() {
        let mut todo = Todo::new(
            "Test title".to_string(),
            "Test description".to_string(),
            Priority::High,
            1695844953,
        );
        todo.start();
        todo.complete();
        assert_eq!(todo.status, Status::Completed);
    }

    #[test]
    fn test_complete_not_started_todo() {
        let mut todo = Todo::new(
            "Test title".to_string(),
            "Test description".to_string(),
            Priority::High,
            1695844953,
        );
        todo.complete();
        assert_ne!(todo.status, Status::Completed);
    }
}
