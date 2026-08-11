// car_diary.js — JavaScript версия

const fs = require('fs');
const readline = require('readline');

const DATA_FILE = 'repairs.json';
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

let repairs = [];

function load() {
    if (fs.existsSync(DATA_FILE)) {
        try {
            repairs = JSON.parse(fs.readFileSync(DATA_FILE, 'utf8'));
        } catch (e) {
            repairs = [];
        }
    } else {
        repairs = [];
    }
}

function save() {
    fs.writeFileSync(DATA_FILE, JSON.stringify(repairs, null, 2));
}

function ask(question) {
    return new Promise(resolve => rl.question(question, resolve));
}

function color(text, code) {
    return `\x1b[${code}m${text}\x1b[0m`;
}

async function main() {
    load();
    while (true) {
        console.log(`\n${color('🚗 Дневник автомобиля (JavaScript)', '36')}`);
        console.log("1. Добавить запись");
        console.log("2. Показать все записи");
        console.log("3. Поиск записей");
        console.log("4. Удалить запись");
        console.log("5. Статистика");
        console.log("6. Выход");
        const choice = await ask("Выберите действие: ");
        switch (choice.trim()) {
            case "1": await addRepair(); break;
            case "2": listAll(); break;
            case "3": await searchRepairs(); break;
            case "4": await deleteRepair(); break;
            case "5": stats(); break;
            case "6": console.log("До свидания!"); rl.close(); return;
            default: console.log(color("Неверный выбор.", "31"));
        }
    }
}

async function addRepair() {
    const date = await ask("Дата (ГГГГ-ММ-ДД): ");
    const mileage = parseInt(await ask("Пробег (км): "));
    const desc = await ask("Описание: ");
    const cost = parseFloat(await ask("Стоимость (руб): "));
    const repair = {
        id: repairs.length + 1,
        date: date.trim(),
        mileage: mileage,
        description: desc.trim(),
        cost: cost
    };
    repairs.push(repair);
    save();
    console.log(color(`✅ Запись добавлена (ID: ${repair.id})`, "32"));
}

function listAll() {
    if (repairs.length === 0) {
        console.log(color("Нет записей.", "33"));
        return;
    }
    console.log(color(`${'ID'.padEnd(4)} ${'Дата'.padEnd(12)} ${'Пробег'.padEnd(10)} ${'Описание'.padEnd(30)} ${'Стоимость'.padStart(10)}`, "36"));
    console.log("-".repeat(70));
    for (const r of repairs) {
        console.log(`${String(r.id).padEnd(4)} ${r.date.padEnd(12)} ${String(r.mileage).padEnd(10)} ${r.description.padEnd(30)} ${String(r.cost.toFixed(2)).padStart(10)}`);
    }
}

async function searchRepairs() {
    const keyword = (await ask("Введите ключевое слово: ")).trim().toLowerCase();
    const results = repairs.filter(r => 
        r.description.toLowerCase().includes(keyword) ||
        r.date.includes(keyword) ||
        String(r.mileage).includes(keyword)
    );
    if (results.length === 0) {
        console.log(color("Ничего не найдено.", "33"));
    } else {
        results.forEach(r => console.log(`${r.id}: ${r.date} | ${r.mileage} км | ${r.description} | ${r.cost} руб.`));
    }
}

async function deleteRepair() {
    listAll();
    const id = parseInt(await ask("Введите ID записи для удаления: "));
    const index = repairs.findIndex(r => r.id === id);
    if (index !== -1) {
        repairs.splice(index, 1);
        save();
        console.log(color("✅ Запись удалена.", "32"));
    } else {
        console.log(color("❌ Запись не найдена.", "31"));
    }
}

function stats() {
    if (repairs.length === 0) {
        console.log("Нет данных.");
        return;
    }
    const total = repairs.reduce((sum, r) => sum + r.cost, 0);
    console.log(color(`Общая стоимость ремонтов: ${total.toFixed(2)} руб.`, "32"));
    console.log(`Средняя стоимость ремонта: ${(total / repairs.length).toFixed(2)} руб.`);
    if (repairs.length > 1) {
        const intervals = [];
        for (let i = 1; i < repairs.length; i++) {
            intervals.push(repairs[i].mileage - repairs[i-1].mileage);
        }
        const avgInterval = intervals.reduce((a,b) => a+b, 0) / intervals.length;
        console.log(`Средний интервал между ремонтами: ${avgInterval.toFixed(1)} км`);
    }
}

main().catch(console.error);
