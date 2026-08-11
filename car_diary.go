// car_diary.go — Go версия

package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Repair struct {
	ID          int     `json:"id"`
	Date        string  `json:"date"`
	Mileage     int     `json:"mileage"`
	Description string  `json:"description"`
	Cost        float64 `json:"cost"`
}

type Diary struct {
	Repairs []Repair
	file    string
}

func NewDiary(file string) *Diary {
	d := &Diary{file: file}
	d.load()
	return d
}

func (d *Diary) load() {
	data, err := os.ReadFile(d.file)
	if err != nil {
		d.Repairs = []Repair{}
		return
	}
	json.Unmarshal(data, &d.Repairs)
}

func (d *Diary) save() {
	data, _ := json.MarshalIndent(d.Repairs, "", "  ")
	os.WriteFile(d.file, data, 0644)
}

func (d *Diary) add(date string, mileage int, desc string, cost float64) int {
	id := len(d.Repairs) + 1
	d.Repairs = append(d.Repairs, Repair{ID: id, Date: date, Mileage: mileage, Description: desc, Cost: cost})
	d.save()
	return id
}

func (d *Diary) list() {
	if len(d.Repairs) == 0 {
		fmt.Println("Нет записей.")
		return
	}
	fmt.Printf("%-4s %-12s %-10s %-30s %10s\n", "ID", "Дата", "Пробег", "Описание", "Стоимость")
	fmt.Println(strings.Repeat("-", 70))
	for _, r := range d.Repairs {
		fmt.Printf("%-4d %-12s %-10d %-30s %10.2f\n", r.ID, r.Date, r.Mileage, r.Description, r.Cost)
	}
}

func (d *Diary) search(keyword string) {
	found := false
	for _, r := range d.Repairs {
		if strings.Contains(r.Description, keyword) || strings.Contains(r.Date, keyword) || strconv.Itoa(r.Mileage) == keyword {
			fmt.Printf("%d: %s | %d км | %s | %.2f руб.\n", r.ID, r.Date, r.Mileage, r.Description, r.Cost)
			found = true
		}
	}
	if !found {
		fmt.Println("Ничего не найдено.")
	}
}

func (d *Diary) delete(id int) bool {
	for i, r := range d.Repairs {
		if r.ID == id {
			d.Repairs = append(d.Repairs[:i], d.Repairs[i+1:]...)
			d.save()
			return true
		}
	}
	return false
}

func (d *Diary) stats() {
	if len(d.Repairs) == 0 {
		fmt.Println("Нет данных.")
		return
	}
	var total float64
	for _, r := range d.Repairs {
		total += r.Cost
	}
	fmt.Printf("Общая стоимость: %.2f руб.\n", total)
	fmt.Printf("Средняя стоимость: %.2f руб.\n", total/float64(len(d.Repairs)))
	if len(d.Repairs) > 1 {
		var intervals []int
		for i := 1; i < len(d.Repairs); i++ {
			intervals = append(intervals, d.Repairs[i].Mileage-d.Repairs[i-1].Mileage)
		}
		var sum int
		for _, v := range intervals {
			sum += v
		}
		fmt.Printf("Средний интервал между ремонтами: %.1f км\n", float64(sum)/float64(len(intervals)))
	}
}

func main() {
	diary := NewDiary("repairs.json")
	reader := bufio.NewReader(os.Stdin)
	for {
		fmt.Println("\n🚗 Дневник автомобиля (Go)")
		fmt.Println("1. Добавить запись")
		fmt.Println("2. Показать все записи")
		fmt.Println("3. Поиск записей")
		fmt.Println("4. Удалить запись")
		fmt.Println("5. Статистика")
		fmt.Println("6. Выход")
		fmt.Print("Выберите действие: ")
		choice, _ := reader.ReadString('\n')
		choice = strings.TrimSpace(choice)
		switch choice {
		case "1":
			fmt.Print("Дата (ГГГГ-ММ-ДД): ")
			date, _ := reader.ReadString('\n')
			date = strings.TrimSpace(date)
			fmt.Print("Пробег (км): ")
			mileageStr, _ := reader.ReadString('\n')
			mileage, _ := strconv.Atoi(strings.TrimSpace(mileageStr))
			fmt.Print("Описание: ")
			desc, _ := reader.ReadString('\n')
			desc = strings.TrimSpace(desc)
			fmt.Print("Стоимость (руб): ")
			costStr, _ := reader.ReadString('\n')
			cost, _ := strconv.ParseFloat(strings.TrimSpace(costStr), 64)
			id := diary.add(date, mileage, desc, cost)
			fmt.Printf("✅ Запись добавлена (ID: %d)\n", id)
		case "2":
			diary.list()
		case "3":
			fmt.Print("Введите ключевое слово: ")
			keyword, _ := reader.ReadString('\n')
			keyword = strings.TrimSpace(keyword)
			diary.search(keyword)
		case "4":
			diary.list()
			fmt.Print("Введите ID записи для удаления: ")
			idStr, _ := reader.ReadString('\n')
			id, _ := strconv.Atoi(strings.TrimSpace(idStr))
			if diary.delete(id) {
				fmt.Println("✅ Запись удалена.")
			} else {
				fmt.Println("❌ Запись не найдена.")
			}
		case "5":
			diary.stats()
		case "6":
			fmt.Println("До свидания!")
			return
		default:
			fmt.Println("Неверный выбор.")
		}
	}
}
