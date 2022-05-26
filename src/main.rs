// Получение из стандартной библиотеки std библиотеку io.
use std::io;

fn main() { // Функция main - точка входя для проекта guessing_game.
    println!("Guess a number!");
    println!("Please input youre guess.");
    
//   ключевое слово для создания переменной
//   |   определение, что переменная должна быть изменяемой (по умолчанию неизменяемая)
//   |   |    наименование переменной
//   |   |    |   знак привязывания перемнной
//   |   |    |   |       создание инстанса String - типа данных для строк (кодировка UTF-8)
//   |   |    |   |       | 
//   V   V    V   V       V
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
