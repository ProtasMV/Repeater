use std::io::{self, Write};
use colored::*;
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
    println!("Made by: @ProtasMV");
    print!("{}", "What would you like to enter? ".yellow());
    flush();

    let user_messege = input();

    let user_quantity = loop {    

        print!("{}", "How many times would you like to print it? ".yellow());
        flush();
    
        let user_quantity: i32 = match input().trim().parse() {
            Ok(quantity) => quantity,
            Err(er) => {println!("An error occurred: {er}, please try again"); continue}
        };
        
        break user_quantity;
    };

    (user_messege, user_quantity)
}

fn flush() {
    match io::stdout().flush() {
        Ok(_) =>{},
        Err(er) => {println!("An error occurred: {er}")}    
    }    
}

fn input() -> String{
    let mut data = String::new();
    match io::stdin().read_line(&mut data) {
        Ok(_) => {},
        Err(er) => println!("An error occurred: {er}")        
    };
    data
}

fn want_cont() -> bool {
    let mut user_cont = String::new();

    loop {
        println!();
        print!("{}", "Continue? (Yes/No): ".yellow());
        flush();

        match io::stdin().read_line(&mut user_cont) {
            Ok(_) => {},
            Err(er) => println!("An error occurred: {er}")
        }

        match user_cont.trim().to_lowercase().as_str() {
            "1"|"yes"|"y" => {user_cont.clear(); break true},
            "2"|"no"|"n" => {user_cont.clear(); break false},
            _=> {user_cont.clear(); println!("Неудалось распознать ваш ввод, попробуйте ещё раз"); continue},
        }
    }
}