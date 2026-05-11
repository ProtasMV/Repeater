use std::io::{self, Write};

fn main() {
    
    loop {
        let mut cycles = 0;    
        
        let (user_messege, user_quantity) = user_parameters();

        while cycles < user_quantity {
            println!("{}", user_messege.trim());
            cycles += 1;
        }
    
        if !want_cont() {
            break;
        }
    }
}

fn user_parameters() -> (String, i32) {
    print!("Что бы вы хотел ввести? ");
    flush();

    let user_messege = input();

    let user_quantity = loop {    

        print!("Сколько раз вы бы хотели ето вывести? ");
        flush();
    
        let user_quantity: i32 = match input().trim().parse() {
            Ok(quantity) => quantity,
            Err(er) => {println!("Произошла ошибка: {er}, попробуйте ещё раз"); continue}
        };
        
        break user_quantity;
    };

    (user_messege, user_quantity)
}

fn flush() {
    match io::stdout().flush() {
        Ok(_) =>{},
        Err(er) => {println!("Произошла ошибка: {er}, выход с программы...")}    
    }    
}

fn input() -> String{
    let mut data = String::new();
    match io::stdin().read_line(&mut data) {
        Ok(_) => {},
        Err(er) => println!("Произошла ошибка: {er}")        
    };
    data
}

fn want_cont() -> bool {
    let mut user_cont = String::new();

    loop {
        println!();
        print!("Продолжить? (Да/Нет): ");
        flush();

        match io::stdin().read_line(&mut user_cont) {
            Ok(_) => {},
            Err(er) => println!("Произошла ошибка: {er}")
        }

        match user_cont.trim().to_lowercase().as_str() {
            "1"|"да" => {user_cont.clear(); break true},
            "2"|"нет" => {user_cont.clear(); break false},
            _=> {user_cont.clear(); println!("Неудалось распознать ваш ввод, попробуйте ещё раз"); continue},
        }
    }
}