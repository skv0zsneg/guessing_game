// Типаж Rng определяет методы, реализующие генераторы случайных чисел,
// и этот типаж должен быть в области видимости, чтобы можно было
// использовать эти методы.
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number!");

    // специальный генератор случайных чисел, который мы собираемся
    // использовать: локальный для текущего потока выполнения и
    // заполняемый операционной системой
    //                             |
    //                             |
    //                             |    принимает выражение диапазона в качестве аргумента и генерирует
    //                             |    случайное число в пределах диапазона. Выражение диапазона,
    //                             |    которое здесь используется, имеет форму start...end и является
    //                             |    инклюзивным по нижней границе, но эксклюзивным по верхней,
    //                             |    поэтому нужно указать 1...101, чтобы запросить число от 1 до 100.
    //                             |    Как вариант, можно передать диапазон 1..=100, что будет
    //                             |    эквивалентно.
    //                             |          |
    //                             V          V
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input youre guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    //   затеняем старое значение guess новым
    //    |
    //    V
    //                обращение к старой переменной guess
    //                 |
    //                 V
    let guess: u32 = guess
        .trim() // удаление пробельных символов в начале и конце строки guess
        .parse() // метод преобразует строку к числу - для этого у новой переменной guess аннотирован тип guess: u32
        .expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
