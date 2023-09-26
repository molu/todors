use phf::phf_map;

pub static PRIORITY_MAP: phf::Map<&'static str, Priority> = phf_map! {
    "P1" => Priority::High,
    "P2" => Priority::Medium,
    "P3" => Priority::Low,
};

#[derive(Debug)]
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

        if priority != self.priority {
            self.priority = priority;
        }

        if due_date > 0 {
            self.due_date = due_date;
        } else {
            println!("Due date not updated ({})", due_date);
        }
    }
}
