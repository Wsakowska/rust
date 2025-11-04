use std::io;

struct Task {
    id: usize,
    description: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("Dodano zadanie");
    }

    fn list(&self) {
        if self.tasks.is_empty() {
            println!("Lista pusta");
            return;
        }

        for task in &self.tasks {
            let status = if task.completed { "X" } else { " " };
            println!("[{}] {}. {}", status, task.id, task.description);
        }
    }

    fn complete(&mut self, id: usize) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("Zadanie ukonczone");
                return;
            }
        }
        println!("Nie znaleziono");
    }

    fn delete(&mut self, id: usize) {
        let len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        
        if self.tasks.len() < len {
            println!("Usunieto");
        } else {
            println!("Nie znaleziono");
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut todo = TodoList::new();

    loop {
        println!("\n1. Dodaj");
        println!("2. Lista");
        println!("3. Ukoncz");
        println!("4. Usun");
        println!("5. Wyjscie");
        println!("Wybierz: ");

        let choice = get_input();

        match choice.as_str() {
            "1" => {
                println!("Opis:");
                let desc = get_input();
                if !desc.is_empty() {
                    todo.add(desc);
                }
            }
            "2" => {
                todo.list();
            }
            "3" => {
                println!("ID:");
                let id = get_input().parse::<usize>().unwrap_or(0);
                todo.complete(id);
            }
            "4" => {
                println!("ID:");
                let id = get_input().parse::<usize>().unwrap_or(0);
                todo.delete(id);
            }
            "5" => {
                break;
            }
            _ => {
                println!("Zly wybor");
            }
        }
    }
}