// car_diary.cs — C# версия

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;

namespace CarDiary
{
    class Repair
    {
        public int Id { get; set; }
        public string Date { get; set; }
        public int Mileage { get; set; }
        public string Description { get; set; }
        public double Cost { get; set; }
    }

    class Program
    {
        private static List<Repair> repairs = new List<Repair>();
        private const string DataFile = "repairs.json";

        static void Main()
        {
            Load();
            while (true)
            {
                Console.WriteLine("\n\u001B[36m🚗 Дневник автомобиля (C#)\u001B[0m");
                Console.WriteLine("1. Добавить запись");
                Console.WriteLine("2. Показать все записи");
                Console.WriteLine("3. Поиск записей");
                Console.WriteLine("4. Удалить запись");
                Console.WriteLine("5. Статистика");
                Console.WriteLine("6. Выход");
                Console.Write("Выберите действие: ");
                string choice = Console.ReadLine();
                switch (choice)
                {
                    case "1": AddRepair(); break;
                    case "2": ListAll(); break;
                    case "3": SearchRepairs(); break;
                    case "4": DeleteRepair(); break;
                    case "5": Stats(); break;
                    case "6": Console.WriteLine("До свидания!"); return;
                    default: Console.WriteLine("\u001B[31mНеверный выбор.\u001B[0m"); break;
                }
            }
        }

        static void Load()
        {
            if (File.Exists(DataFile))
            {
                try
                {
                    string json = File.ReadAllText(DataFile);
                    repairs = JsonSerializer.Deserialize<List<Repair>>(json) ?? new List<Repair>();
                }
                catch
                {
                    repairs = new List<Repair>();
                }
            }
        }

        static void Save()
        {
            string json = JsonSerializer.Serialize(repairs, new JsonSerializerOptions { WriteIndented = true });
            File.WriteAllText(DataFile, json);
        }

        static void AddRepair()
        {
            Console.Write("Дата (ГГГГ-ММ-ДД): ");
            string date = Console.ReadLine();
            Console.Write("Пробег (км): ");
            int mileage = int.Parse(Console.ReadLine());
            Console.Write("Описание: ");
            string desc = Console.ReadLine();
            Console.Write("Стоимость (руб): ");
            double cost = double.Parse(Console.ReadLine());
            var repair = new Repair
            {
                Id = repairs.Count + 1,
                Date = date,
                Mileage = mileage,
                Description = desc,
                Cost = cost
            };
            repairs.Add(repair);
            Save();
            Console.WriteLine($"\u001B[32m✅ Запись добавлена (ID: {repair.Id})\u001B[0m");
        }

        static void ListAll()
        {
            if (repairs.Count == 0)
            {
                Console.WriteLine("\u001B[33mНет записей.\u001B[0m");
                return;
            }
            Console.WriteLine($"\u001B[36m{"ID",-4} {"Дата",-12} {"Пробег",-10} {"Описание",-30} {"Стоимость",10}\u001B[0m");
            Console.WriteLine(new string('-', 70));
            foreach (var r in repairs)
                Console.WriteLine($"{r.Id,-4} {r.Date,-12} {r.Mileage,-10} {r.Description,-30} {r.Cost,10:F2}");
        }

        static void SearchRepairs()
        {
            Console.Write("Введите ключевое слово: ");
            string keyword = Console.ReadLine().ToLower();
            var results = repairs.Where(r =>
                r.Description.ToLower().Contains(keyword) ||
                r.Date.Contains(keyword) ||
                r.Mileage.ToString() == keyword
            ).ToList();
            if (results.Count == 0)
                Console.WriteLine("\u001B[33mНичего не найдено.\u001B[0m");
            else
                results.ForEach(r => Console.WriteLine($"{r.Id}: {r.Date} | {r.Mileage} км | {r.Description} | {r.Cost} руб."));
        }

        static void DeleteRepair()
        {
            ListAll();
            Console.Write("Введите ID записи для удаления: ");
            int id = int.Parse(Console.ReadLine());
            var item = repairs.FirstOrDefault(r => r.Id == id);
            if (item != null)
            {
                repairs.Remove(item);
                Save();
                Console.WriteLine("\u001B[32m✅ Запись удалена.\u001B[0m");
            }
            else
                Console.WriteLine("\u001B[31m❌ Запись не найдена.\u001B[0m");
        }

        static void Stats()
        {
            if (repairs.Count == 0)
            {
                Console.WriteLine("Нет данных.");
                return;
            }
            double total = repairs.Sum(r => r.Cost);
            Console.WriteLine($"\u001B[32mОбщая стоимость ремонтов: {total:F2} руб.\u001B[0m");
            Console.WriteLine($"Средняя стоимость ремонта: {total / repairs.Count:F2} руб.");
            if (repairs.Count > 1)
            {
                var intervals = new List<int>();
                for (int i = 1; i < repairs.Count; i++)
                    intervals.Add(repairs[i].Mileage - repairs[i-1].Mileage);
                double avgInterval = intervals.Average();
                Console.WriteLine($"Средний интервал между ремонтами: {avgInterval:F1} км");
            }
        }
    }
}
