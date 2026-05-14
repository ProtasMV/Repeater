use std::io::{self, Write};
use colored::*;

const SYS_COLOR: Color = Color::TrueColor {r: 255, g: 255, b: 0};
const AUTOR_COLOR: Color = Color::TrueColor {r: 255, g: 165, b: 0};

fn main() {
    println!("{}", "Made by: @ProtasMV".color(AUTOR_COLOR).bold());    
    loop {
        let (user_messege, user_quantity) = user_parameters();
        
        for _ in 0..user_quantity {
            println!("{}", user_messege.trim());
        }
    
        if !want_cont() {
            break;
        }
    }
}

fn user_parameters() -> (String, u32) {
    print!("{}", "What would you like to enter? ".color(SYS_COLOR));
    flush();

    let user_messege = input();
    let user_quantity = loop {    

        print!("{}", "How many times would you like to print it? ".color(SYS_COLOR));
        flush();
    
        let user_quantity: u32 = match input().trim().parse() {
            Ok(quantity) => quantity,
            Err(er) => {println!("An error occurred: {er}, please try again"); continue}
        };
        
        break user_quantity;
    };

    println!();    
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
    loop {
        println!();
        print!("{}", "Continue? (Yes/No): ".color(SYS_COLOR));
        flush();

        let mut user_cont = input();

        match user_cont.trim().to_lowercase().as_str() {
            "1"|"yes"|"y" => {user_cont.clear(); break true},
            "2"|"no"|"n" => {user_cont.clear(); break false},
            _=> {user_cont.clear(); println!("Could not recognize your input, please try again"); continue},
        }
    }
}