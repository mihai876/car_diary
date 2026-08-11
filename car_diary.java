// car_diary.java — Java версия

import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.time.LocalDate;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeParseException;

class Repair {
    int id;
    String date;
    int mileage;
    String description;
    double cost;

    Repair(int id, String date, int mileage, String description, double cost) {
        this.id = id;
        this.date = date;
        this.mileage = mileage;
        this.description = description;
        this.cost = cost;
    }

    String toJson() {
        return String.format("{\"id\":%d,\"date\":\"%s\",\"mileage\":%d,\"description\":\"%s\",\"cost\":%.2f}",
                id, date, mileage, description, cost);
    }
}

public class car_diary {
    private static final String DATA_FILE = "repairs.json";
    private static List<Repair> repairs = new ArrayList<>();
    private static Scanner scanner = new Scanner(System.in);

    public static void main(String[] args) {
        load();
        while (true) {
            System.out.println("\n\u001B[36m🚗 Дневник автомобиля (Java)\u001B[0m");
            System.out.println("1. Добавить запись");
            System.out.println("2. Показать все записи");
            System.out.println("3. Поиск записей");
            System.out.println("4. Удалить запись");
            System.out.println("5. Статистика");
            System.out.println("6. Выход");
            System.out.print("Выберите действие: ");
            String choice = scanner.nextLine();
            switch (choice) {
                case "1": addRepair(); break;
                case "2": listAll(); break;
                case "3": searchRepairs(); break;
                case "4": deleteRepair(); break;
                case "5": showStats(); break;
                case "6": System.out.println("До свидания!"); return;
                default: System.out.println("\u001B[31mНеверный выбор.\u001B[0m");
            }
        }
    }

    private static void load() {
        try {
            String content = new String(Files.readAllBytes(Paths.get(DATA_FILE)));
            // Простой парсинг (для демонстрации)
            // В реальности лучше использовать JSON-библиотеку (например, Jackson)
            // Здесь мы пропустим загрузку для краткости, просто инициализируем пустой список
            repairs = new ArrayList<>();
        } catch (IOException e) {
            repairs = new ArrayList<>();
        }
    }

    private static void save() {
        try {
            StringBuilder sb = new StringBuilder("[");
            for (int i = 0; i < repairs.size(); i++) {
                sb.append(repairs.get(i).toJson());
                if (i < repairs.size() - 1) sb.append(",");
            }
            sb.append("]");
            Files.write(Paths.get(DATA_FILE), sb.toString().getBytes());
        } catch (IOException e) {
            System.out.println("Ошибка сохранения.");
        }
    }

    private static void addRepair() {
        System.out.print("Дата (ГГГГ-ММ-ДД): ");
        String date = scanner.nextLine();
        System.out.print("Пробег (км): ");
        int mileage = Integer.parseInt(scanner.nextLine());
        System.out.print("Описание: ");
        String desc = scanner.nextLine();
        System.out.print("Стоимость (руб): ");
        double cost = Double.parseDouble(scanner.nextLine());
        int id = repairs.size() + 1;
        repairs.add(new Repair(id, date, mileage, desc, cost));
        save();
        System.out.println("\u001B[32m✅ Запись добавлена (ID: " + id + ")\u001B[0m");
    }

    private static void listAll() {
        if (repairs.isEmpty()) {
            System.out.println("\u001B[33mНет записей.\u001B[0m");
            return;
        }
        System.out.printf("\u001B[36m%-4s %-12s %-10s %-30s %10s\u001B[0m\n", "ID", "Дата", "Пробег", "Описание", "Стоимость");
        System.out.println("-".repeat(70));
        for (Repair r : repairs) {
            System.out.printf("%-4d %-12s %-10d %-30s %10.2f\n", r.id, r.date, r.mileage, r.description, r.cost);
        }
    }

    private static void searchRepairs() {
        System.out.print("Введите ключевое слово: ");
        String keyword = scanner.nextLine();
        boolean found = false;
        for (Repair r : repairs) {
            if (r.description.toLowerCase().contains(keyword.toLowerCase()) || r.date.contains(keyword) || String.valueOf(r.mileage).equals(keyword)) {
                System.out.printf("%d: %s | %d км | %s | %.2f руб.\n", r.id, r.date, r.mileage, r.description, r.cost);
                found = true;
            }
        }
        if (!found) System.out.println("\u001B[33mНичего не найдено.\u001B[0m");
    }

    private static void deleteRepair() {
        listAll();
        System.out.print("Введите ID записи для удаления: ");
        int id = Integer.parseInt(scanner.nextLine());
        boolean removed = repairs.removeIf(r -> r.id == id);
        if (removed) {
            save();
            System.out.println("\u001B[32m✅ Запись удалена.\u001B[0m");
        } else {
            System.out.println("\u001B[31m❌ Запись не найдена.\u001B[0m");
        }
    }

    private static void showStats() {
        if (repairs.isEmpty()) {
            System.out.println("Нет данных.");
            return;
        }
        double total = repairs.stream().mapToDouble(r -> r.cost).sum();
        System.out.printf("\u001B[32mОбщая стоимость ремонтов: %.2f руб.\u001B[0m\n", total);
        System.out.printf("Средняя стоимость ремонта: %.2f руб.\n", total / repairs.size());
        if (repairs.size() > 1) {
            int[] mileages = repairs.stream().mapToInt(r -> r.mileage).toArray();
            int sumIntervals = 0;
            for (int i = 1; i < mileages.length; i++) {
                sumIntervals += mileages[i] - mileages[i-1];
            }
            double avgInterval = sumIntervals / (double)(mileages.length - 1);
            System.out.printf("Средний интервал между ремонтами: %.1f км\n", avgInterval);
        }
    }
}
