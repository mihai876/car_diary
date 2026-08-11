<?php
// car_diary.php — PHP версия

$dataFile = 'repairs.json';

function loadRepairs() {
    global $dataFile;
    if (file_exists($dataFile)) {
        $json = file_get_contents($dataFile);
        return json_decode($json, true) ?: [];
    }
    return [];
}

function saveRepairs($repairs) {
    global $dataFile;
    file_put_contents($dataFile, json_encode($repairs, JSON_PRETTY_PRINT | JSON_UNESCAPED_UNICODE));
}

$repairs = loadRepairs();

function color($text, $code) {
    return "\033[{$code}m{$text}\033[0m";
}

while (true) {
    echo "\n" . color("🚗 Дневник автомобиля (PHP)", '36') . "\n";
    echo "1. Добавить запись\n";
    echo "2. Показать все записи\n";
    echo "3. Поиск записей\n";
    echo "4. Удалить запись\n";
    echo "5. Статистика\n";
    echo "6. Выход\n";
    echo "Выберите действие: ";
    $choice = trim(fgets(STDIN));

    switch ($choice) {
        case '1':
            echo "Дата (ГГГГ-ММ-ДД): ";
            $date = trim(fgets(STDIN));
            echo "Пробег (км): ";
            $mileage = (int) trim(fgets(STDIN));
            echo "Описание: ";
            $desc = trim(fgets(STDIN));
            echo "Стоимость (руб): ";
            $cost = (float) trim(fgets(STDIN));
            $id = count($repairs) + 1;
            $repairs[] = [
                'id' => $id,
                'date' => $date,
                'mileage' => $mileage,
                'description' => $desc,
                'cost' => $cost
            ];
            saveRepairs($repairs);
            echo color("✅ Запись добавлена (ID: $id)\n", '32');
            break;

        case '2':
            if (empty($repairs)) {
                echo color("Нет записей.\n", '33');
            } else {
                printf(color("%-4s %-12s %-10s %-30s %10s\n", '36'), "ID", "Дата", "Пробег", "Описание", "Стоимость");
                echo str_repeat("-", 70) . "\n";
                foreach ($repairs as $r) {
                    printf("%-4d %-12s %-10d %-30s %10.2f\n", $r['id'], $r['date'], $r['mileage'], $r['description'], $r['cost']);
                }
            }
            break;

        case '3':
            echo "Введите ключевое слово: ";
            $keyword = trim(fgets(STDIN));
            $found = false;
            foreach ($repairs as $r) {
                if (stripos($r['description'], $keyword) !== false || strpos($r['date'], $keyword) !== false || $r['mileage'] == $keyword) {
                    echo "{$r['id']}: {$r['date']} | {$r['mileage']} км | {$r['description']} | {$r['cost']} руб.\n";
                    $found = true;
                }
            }
            if (!$found) echo color("Ничего не найдено.\n", '33');
            break;

        case '4':
            if (empty($repairs)) {
                echo color("Нет записей для удаления.\n", '33');
                break;
            }
            // показать список
            foreach ($repairs as $r) {
                echo "{$r['id']}: {$r['date']} | {$r['description']}\n";
            }
            echo "Введите ID записи для удаления: ";
            $id = (int) trim(fgets(STDIN));
            $index = array_search($id, array_column($repairs, 'id'));
            if ($index !== false) {
                array_splice($repairs, $index, 1);
                saveRepairs($repairs);
                echo color("✅ Запись удалена.\n", '32');
            } else {
                echo color("❌ Запись не найдена.\n", '31');
            }
            break;

        case '5':
            if (empty($repairs)) {
                echo "Нет данных.\n";
                break;
            }
            $total = array_sum(array_column($repairs, 'cost'));
            echo color("Общая стоимость ремонтов: {$total} руб.\n", '32');
            echo "Средняя стоимость ремонта: " . ($total / count($repairs)) . " руб.\n";
            if (count($repairs) > 1) {
                $mileages = array_column($repairs, 'mileage');
                $intervals = [];
                for ($i = 1; $i < count($mileages); $i++) {
                    $intervals[] = $mileages[$i] - $mileages[$i-1];
                }
                $avgInterval = array_sum($intervals) / count($intervals);
                echo "Средний интервал между ремонтами: {$avgInterval} км\n";
            }
            break;

        case '6':
            echo "До свидания!\n";
            exit(0);

        default:
            echo color("Неверный выбор.\n", '31');
    }
}
?>
