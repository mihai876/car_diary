# car_diary.py — Python версия

import json
import os
from datetime import datetime
from colorama import init, Fore, Style

init(autoreset=True)
DATA_FILE = "repairs.json"

class CarDiary:
    def __init__(self):
        self.repairs = []
        self.load()

    def load(self):
        if os.path.exists(DATA_FILE):
            try:
                with open(DATA_FILE, 'r', encoding='utf-8') as f:
                    self.repairs = json.load(f)
            except:
                self.repairs = []

    def save(self):
        with open(DATA_FILE, 'w', encoding='utf-8') as f:
            json.dump(self.repairs, f, indent=2, ensure_ascii=False)

    def add(self, date, mileage, desc, cost):
        repair = {
            "id": len(self.repairs) + 1,
            "date": date,
            "mileage": mileage,
            "description": desc,
            "cost": cost
        }
        self.repairs.append(repair)
        self.save()
        return repair["id"]

    def list_all(self):
        if not self.repairs:
            print(Fore.YELLOW + "Нет записей.")
            return
        print(Fore.CYAN + f"{'ID':<4} {'Дата':<12} {'Пробег':<10} {'Описание':<30} {'Стоимость':>10}")
        print("-" * 70)
        for r in self.repairs:
            print(f"{r['id']:<4} {r['date']:<12} {r['mileage']:<10} {r['description']:<30} {r['cost']:>10}")

    def search(self, keyword):
        results = [r for r in self.repairs if keyword.lower() in r['description'].lower() or keyword in r['date'] or str(r['mileage']) == keyword]
        if not results:
            print(Fore.YELLOW + "Ничего не найдено.")
        else:
            for r in results:
                print(f"{r['id']}: {r['date']} | {r['mileage']} км | {r['description']} | {r['cost']} руб.")

    def delete(self, id):
        for i, r in enumerate(self.repairs):
            if r['id'] == id:
                del self.repairs[i]
                self.save()
                return True
        return False

    def stats(self):
        total = sum(r['cost'] for r in self.repairs)
        print(Fore.GREEN + f"Общая стоимость ремонтов: {total} руб.")
        if self.repairs:
            avg = total / len(self.repairs)
            print(f"Средняя стоимость ремонта: {avg:.2f} руб.")
            # пробеги
            mileages = [r['mileage'] for r in self.repairs]
            if len(mileages) > 1:
                intervals = [mileages[i] - mileages[i-1] for i in range(1, len(mileages))]
                avg_interval = sum(intervals) / len(intervals)
                print(f"Средний интервал между ремонтами: {avg_interval:.1f} км")
        else:
            print("Нет данных для статистики.")

def main():
    diary = CarDiary()
    while True:
        print(Fore.CYAN + "\n🚗 Дневник автомобиля (Python)")
        print("1. Добавить запись")
        print("2. Показать все записи")
        print("3. Поиск записей")
        print("4. Удалить запись")
        print("5. Статистика")
        print("6. Выход")
        choice = input("Выберите действие: ").strip()
        if choice == "1":
            date = input("Дата (ГГГГ-ММ-ДД): ")
            mileage = int(input("Пробег (км): "))
            desc = input("Описание: ")
            cost = float(input("Стоимость (руб): "))
            id = diary.add(date, mileage, desc, cost)
            print(Fore.GREEN + f"✅ Запись добавлена (ID: {id})")
        elif choice == "2":
            diary.list_all()
        elif choice == "3":
            keyword = input("Введите ключевое слово (дата, пробег, описание): ")
            diary.search(keyword)
        elif choice == "4":
            diary.list_all()
            id = int(input("Введите ID записи для удаления: "))
            if diary.delete(id):
                print(Fore.GREEN + "✅ Запись удалена.")
            else:
                print(Fore.RED + "❌ Запись не найдена.")
        elif choice == "5":
            diary.stats()
        elif choice == "6":
            print("До свидания!")
            break
        else:
            print(Fore.RED + "Неверный выбор, попробуйте снова.")

if __name__ == "__main__":
    main()
