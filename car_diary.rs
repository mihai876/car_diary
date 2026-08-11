// car_diary.rs — Rust версия

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Clone)]
struct Repair {
    id: usize,
    date: String,
    mileage: u32,
    description: String,
    cost: f64,
}

struct Diary {
    repairs: Vec<Repair>,
    file: String,
}

impl Diary {
    fn new(file: &str) -> Self {
        let mut d = Diary { repairs: Vec::new(), file: file.to_string() };
        d.load();
        d
    }

    fn load(&mut self) {
        if let Ok(data) = fs::read_to_string(&self.file) {
            if let Ok(repairs) = serde_json::from_str(&data) {
                self.repairs = repairs;
                return;
            }
        }
        self.repairs = Vec::new();
    }

    fn save(&self) {
        let data = serde_json::to_string_pretty(&self.repairs).unwrap();
        fs::write(&self.file, data).unwrap();
    }

    fn add(&mut self, date: String, mileage: u32, description: String, cost: f64) -> usize {
        let id = self.repairs.len() + 1;
        self.repairs.push(Repair { id, date, mileage, description, cost });
        self.save();
        id
    }

    fn list(&self) {
        if self.repairs.is_empty() {
            println!("\x1b[33mНет записей.\x1b[0m");
            return;
        }
        println!("\x1b[36m{:<4} {:<12} {:<10} {:<30} {:>10}\x1b[0m", "ID", "Дата", "Пробег", "Описание", "Стоимость");
        println!("{}", "-".repeat(70));
        for r in &self.repairs {
            println!("{:<4} {:<12} {:<10} {:<30} {:>10.2}", r.id, r.date, r.mileage, r.description, r.cost);
        }
    }

    fn search(&self, keyword: &str) {
        let keyword_lower = keyword.to_lowercase();
        let results: Vec<&Repair> = self.repairs.iter()
            .filter(|r| r.description.to_lowercase().contains(&keyword_lower) ||
                         r.date.contains(keyword) ||
                         r.mileage.to_string() == keyword)
            .collect();
        if results.is_empty() {
            println!("\x1b[33mНичего не найдено.\x1b[0m");
        } else {
            for r in results {
                println!("{}: {} | {} км | {} | {} руб.", r.id, r.date, r.mileage, r.description, r.cost);
            }
        }
    }

    fn delete(&mut self, id: usize) -> bool {
        let pos = self.repairs.iter().position(|r| r.id == id);
        if let Some(idx) = pos {
            self.repairs.remove(idx);
            self.save();
            true
        } else {
            false
        }
    }

    fn stats(&self) {
        if self.repairs.is_empty() {
            println!("Нет данных.");
            return;
        }
        let total: f64 = self.repairs.iter().map(|r| r.cost).sum();
        println!("\x1b[32mОбщая стоимость ремонтов: {:.2} руб.\x1b[0m", total);
        println!("Средняя стоимость ремонта: {:.2} руб.", total / self.repairs.len() as f64);
        if self.repairs.len() > 1 {
            let intervals: Vec<u32> = (1..self.repairs.len())
                .map(|i| self.repairs[i].mileage - self.repairs[i-1].mileage)
                .collect();
            let avg_interval = intervals.iter().sum::<u32>() as f64 / intervals.len() as f64;
            println!("Средний интервал между ремонтами: {:.1} км", avg_interval);
        }
    }
}

fn main() {
    let mut diary = Diary::new("repairs.json");
    loop {
        println!("\n\x1b[36m🚗 Дневник автомобиля (Rust)\x1b[0m");
        println!("1. Добавить запись");
        println!("2. Показать все записи");
        println!("3. Поиск записей");
        println!("4. Удалить запись");
        println!("5. Статистика");
        println!("6. Выход");
        print!("Выберите действие: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                print!("Дата (ГГГГ-ММ-ДД): ");
                io::stdout().flush().unwrap();
                let mut date = String::new();
                io::stdin().read_line(&mut date).unwrap();
                let date = date.trim().to_string();
                print!("Пробег (км): ");
                io::stdout().flush().unwrap();
                let mut mileage_str = String::new();
                io::stdin().read_line(&mut mileage_str).unwrap();
                let mileage: u32 = mileage_str.trim().parse().unwrap();
                print!("Описание: ");
                io::stdout().flush().unwrap();
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                let desc = desc.trim().to_string();
                print!("Стоимость (руб): ");
                io::stdout().flush().unwrap();
                let mut cost_str = String::new();
                io::stdin().read_line(&mut cost_str).unwrap();
                let cost: f64 = cost_str.trim().parse().unwrap();
                let id = diary.add(date, mileage, desc, cost);
                println!("\x1b[32m✅ Запись добавлена (ID: {})\x1b[0m", id);
            }
            "2" => diary.list(),
            "3" => {
                print!("Введите ключевое слово: ");
                io::stdout().flush().unwrap();
                let mut keyword = String::new();
                io::stdin().read_line(&mut keyword).unwrap();
                diary.search(keyword.trim());
            }
            "4" => {
                diary.list();
                print!("Введите ID записи для удаления: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id: usize = id_str.trim().parse().unwrap();
                if diary.delete(id) {
                    println!("\x1b[32m✅ Запись удалена.\x1b[0m");
                } else {
                    println!("\x1b[31m❌ Запись не найдена.\x1b[0m");
                }
            }
            "5" => diary.stats(),
            "6" => {
                println!("До свидания!");
                break;
            }
            _ => println!("\x1b[31mНеверный выбор.\x1b[0m"),
        }
    }
}
