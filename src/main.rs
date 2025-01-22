use std::{fs, io};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: usize,
    title: String,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: usize, title: &str, description: &str) -> Task {
        Task {
            id,
            title: title.to_string(),
            description: description.to_string(),
            completed: false,
        }
    }

    fn to_string(&self) -> String {
        format!("[{}] {}: {} - {}", 
            if self.completed { "X" } else { " " }, 
            self.id, 
            self.title, 
            self.description)
    }
}

struct TaskManager {
    tasks: Vec<Task>,
    file_name: String,
}

impl TaskManager {
    fn new(file_name: &str) -> TaskManager {
        TaskManager {
            tasks: TaskManager::load_tasks(file_name),
            file_name: file_name.to_string(),
        }
    }

    fn load_tasks(file_name: &str) -> Vec<Task> {
        if let Ok(data) = fs::read_to_string(file_name) {
            if !data.is_empty() {
                serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }

    fn save_tasks(&self) {
        let data = serde_json::to_string_pretty(&self.tasks).unwrap();
        fs::write(&self.file_name, data).unwrap();
    }

    fn add_task(&mut self, title: &str, description: &str) {
        let id = if self.tasks.is_empty() { 1 } else { self.tasks.last().unwrap().id + 1 };
        let task = Task::new(id, title, description);
        self.tasks.push(task);
        self.save_tasks();
        println!("Задача '{}' успішно додано!", title);
    }

    fn list_tasks(&self) {
        println!("\nСписок завдань:");
        if self.tasks.is_empty() {
            println!("Немає завдань для відображення.");
        } else {
            for task in &self.tasks {
                println!("{}", task.to_string());
            }
        }
    }

    fn update_task(&mut self, id: usize, title: &str, description: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.title = title.to_string();
            task.description = description.to_string();
            self.save_tasks();
            println!("Задача ##{} успішно оновлено!", id);
        } else {
            println!("Задача с ID ##{} не знайдена.", id);
        }
    }

    fn delete_task(&mut self, id: usize) {
        if let Some(pos) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(pos);
            self.save_tasks();
            println!("Задача ##{} успішно видалена!", id);
        } else {
            println!("Задача с ID ##{} не знайдена.", id);
        }
    }

    fn mark_as_done(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.completed = true;
            self.save_tasks();
            println!("Задача ##{} відзначена як виконана!", id);
        } else {
            println!("Задача с ID ##{} не знайдена.", id);
        }
    }

    fn run(&mut self) {
        loop {
            println!("\nМеню завдань:");
            println!("1. Додати задачу");
            println!("2. Переглянути завдання");
            println!("3. Редагувати завдання");
            println!("4. Видалити завдання");
            println!("5. Відзначити завдання як виконане");
            println!("6. Вийти");

            print!("Виберіть опцію: ");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice: u32 = choice.trim().parse().unwrap();

            match choice {
                1 => {
                    print!("Введіть назву задачі: ");
                    let mut title = String::new();
                    io::stdin().read_line(&mut title).unwrap();

                    print!("Введіть опис задачі: ");
                    let mut description = String::new();
                    io::stdin().read_line(&mut description).unwrap();

                    self.add_task(&title.trim(), &description.trim());
                }
                2 => self.list_tasks(),
                3 => {
                    print!("Введіть ID задачі для редагування: ");
                    let mut id = String::new();
                    io::stdin().read_line(&mut id).unwrap();
                    let id: usize = id.trim().parse().unwrap();

                    print!("Введіть нову назву задачі: ");
                    let mut title = String::new();
                    io::stdin().read_line(&mut title).unwrap();

                    print!("Введіть новий опис завдання: ");
                    let mut description = String::new();
                    io::stdin().read_line(&mut description).unwrap();

                    self.update_task(id, &title.trim(), &description.trim());
                }
                4 => {
                    print!("Введіть ID завдання для видалення: ");
                    let mut id = String::new();
                    io::stdin().read_line(&mut id).unwrap();
                    let id: usize = id.trim().parse().unwrap();
                    self.delete_task(id);
                }
                5 => {
                    print!("Введіть ID завдання для позначки як виконаного: ");
                    let mut id = String::new();
                    io::stdin().read_line(&mut id).unwrap();
                    let id: usize = id.trim().parse().unwrap();
                    self.mark_as_done(id);
                }
                6 => {
                    println!("До побачення!");
                    break;
                }
                _ => println!("Неправильний вибір. Спробуйте знову."),
            }
        }
    }
}

fn main() {
    let mut manager = TaskManager::new("tasks.json");
    manager.run();
}
