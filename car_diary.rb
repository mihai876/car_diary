# car_diary.rb — Ruby версия

require 'json'
require 'date'

DATA_FILE = 'repairs.json'

class CarDiary
  attr_reader :repairs

  def initialize
    @repairs = []
    load_data
  end

  def load_data
    if File.exist?(DATA_FILE)
      begin
        @repairs = JSON.parse(File.read(DATA_FILE), symbolize_names: true)
      rescue
        @repairs = []
      end
    else
      @repairs = []
    end
  end

  def save
    File.write(DATA_FILE, JSON.pretty_generate(@repairs))
  end

  def add(date, mileage, desc, cost)
    repair = {
      id: @repairs.size + 1,
      date: date,
      mileage: mileage,
      description: desc,
      cost: cost
    }
    @repairs << repair
    save
    repair[:id]
  end

  def list
    if @repairs.empty?
      puts "\e[33mНет записей.\e[0m"
      return
    end
    puts "\e[36m%-4s %-12s %-10s %-30s %10s\e[0m" % ["ID", "Дата", "Пробег", "Описание", "Стоимость"]
    puts "-" * 70
    @repairs.each do |r|
      puts "%-4d %-12s %-10d %-30s %10.2f" % [r[:id], r[:date], r[:mileage], r[:description], r[:cost]]
    end
  end

  def search(keyword)
    results = @repairs.select do |r|
      r[:description].downcase.include?(keyword.downcase) ||
      r[:date].include?(keyword) ||
      r[:mileage].to_s == keyword
    end
    if results.empty?
      puts "\e[33mНичего не найдено.\e[0m"
    else
      results.each { |r| puts "#{r[:id]}: #{r[:date]} | #{r[:mileage]} км | #{r[:description]} | #{r[:cost]} руб." }
    end
  end

  def delete(id)
    found = @repairs.find { |r| r[:id] == id }
    if found
      @repairs.delete(found)
      save
      true
    else
      false
    end
  end

  def stats
    if @repairs.empty?
      puts "Нет данных для статистики."
      return
    end
    total = @repairs.sum { |r| r[:cost] }
    puts "\e[32mОбщая стоимость ремонтов: #{total} руб.\e[0m"
    avg = total / @repairs.size
    puts "Средняя стоимость ремонта: #{avg.round(2)} руб."
    if @repairs.size > 1
      mileages = @repairs.map { |r| r[:mileage] }
      intervals = (1...mileages.size).map { |i| mileages[i] - mileages[i-1] }
      avg_interval = intervals.sum / intervals.size.to_f
      puts "Средний интервал между ремонтами: #{avg_interval.round(1)} км"
    end
  end
end

def main
  diary = CarDiary.new
  loop do
    puts "\n\e[36m🚗 Дневник автомобиля (Ruby)\e[0m"
    puts "1. Добавить запись"
    puts "2. Показать все записи"
    puts "3. Поиск записей"
    puts "4. Удалить запись"
    puts "5. Статистика"
    puts "6. Выход"
    print "Выберите действие: "
    choice = gets.chomp
    case choice
    when "1"
      print "Дата (ГГГГ-ММ-ДД): "
      date = gets.chomp
      print "Пробег (км): "
      mileage = gets.chomp.to_i
      print "Описание: "
      desc = gets.chomp
      print "Стоимость (руб): "
      cost = gets.chomp.to_f
      id = diary.add(date, mileage, desc, cost)
      puts "\e[32m✅ Запись добавлена (ID: #{id})\e[0m"
    when "2"
      diary.list
    when "3"
      print "Введите ключевое слово: "
      keyword = gets.chomp
      diary.search(keyword)
    when "4"
      diary.list
      print "Введите ID записи для удаления: "
      id = gets.chomp.to_i
      if diary.delete(id)
        puts "\e[32m✅ Запись удалена.\e[0m"
      else
        puts "\e[31m❌ Запись не найдена.\e[0m"
      end
    when "5"
      diary.stats
    when "6"
      puts "До свидания!"
      break
    else
      puts "\e[31mНеверный выбор.\e[0m"
    end
  end
end

main if __FILE__ == $0
