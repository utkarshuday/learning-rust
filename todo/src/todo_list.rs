use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

const TODO_APP_FILE_NAME: &str = "tasks.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    description: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    tasks: Vec<TodoItem>,
    todo_path: PathBuf,
}

impl TodoList {
    fn init() -> Result<Self, &'static str> {
        let project_dir = ProjectDirs::from("com", "utkarshuday", "todo app")
            .ok_or("Couldn't find valid home directory")?
            .data_local_dir()
            .to_path_buf();

        fs::create_dir_all(&project_dir).map_err(|_| "Coudn't create directory for todo")?;

        let todo_path = project_dir.join(TODO_APP_FILE_NAME);

        Ok(Self {
            tasks: Vec::new(),
            todo_path,
        })
    }

    fn load_tasks(&mut self) -> Result<(), &'static str> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .read(true)
            .open(&self.todo_path)
            .map_err(|_| "Couldn't create or read a file")?;

        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .map_err(|_| "Couldn't read the contents")?;

        let tasks: Vec<TodoItem> = if contents.is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&contents).map_err(|_| "Todo file has data in invalid format")?
        };

        self.tasks = tasks;

        Ok(())
    }

    fn update_tasks(&mut self) -> Result<(), &'static str> {
        let mut file =
            File::create(&self.todo_path).map_err(|_| "Couldn't create a new todo file")?;
        let serialized_tasks = serde_json::to_string(&self.tasks).unwrap();

        file.write_all(serialized_tasks.as_bytes())
            .map_err(|_| "Couldn't write to todo file")?;

        Ok(())
    }

    pub fn new() -> Result<Self, &'static str> {
        let mut todo_list = Self::init()?;
        todo_list.load_tasks()?;

        Ok(todo_list)
    }

    pub fn add(&mut self, task: String) -> Result<(), &'static str> {
        let new_task = TodoItem {
            description: task,
            completed: false,
        };
        self.tasks.push(new_task);

        self.update_tasks()
    }

    fn mark(&mut self, indices: Vec<usize>, is_completed: bool) -> Result<(), &'static str> {
        self.tasks.iter_mut().enumerate().for_each(|(index, item)| {
            let index_to_find = index + 1;
            if indices.contains(&index_to_find) {
                item.completed = is_completed;
            }
        });

        self.update_tasks()
    }

    pub fn check(&mut self, indices: Vec<usize>) -> Result<(), &'static str> {
        self.mark(indices, true)
    }

    pub fn uncheck(&mut self, indices: Vec<usize>) -> Result<(), &'static str> {
        self.mark(indices, false)
    }

    pub fn clear(&mut self) -> Result<(), &'static str> {
        fs::remove_file(&self.todo_path).map_err(|_| "Couldn't delete the todo file")?;

        Ok(())
    }

    pub fn list(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!(
                "{:>2}. {:<30} [{}]",
                index + 1,
                task.description,
                if task.completed { "*" } else { " " },
            );
        }
    }
}
