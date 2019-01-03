use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use term_painter::Color::*;
use term_painter::ToStyle;

fn read_guess() -> u32 {
    let mut str = String::new();
    loop {
        print!("Enter your guess: ");
        io::stdout().flush().expect("stdout.flush()");
        io::stdin().read_line(&mut str).expect("stdin.readline()");
        let num: u32 = match str.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                continue;
            }
        };
        return num;
    }
}

fn play_game() {
    let secret_num = rand::thread_rng().gen_range(1, 101);

    let mut steps = 0;
    loop {
        let guess = read_guess();
        steps = steps + 1;

        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("{}", Green.paint("That's it!"));
                println!("That's it! It took you {} guess(es) to win.", steps);
                return;
            }
            Ordering::Greater => println!("{}", Red.paint("Too big")),
            Ordering::Less => println!("{}", Red.paint("Too small")),
        }
    }
}

fn ask_for_retry() -> bool {
    print!("Do you want to play again (Y/N)? ");
    io::stdout().flush().expect("stdout.flush()");

    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("stdin.readline()");
    str = str.trim().to_uppercase();
    return match str.as_ref() {
        "Y" => true,
        "YES" => true,
        _ => false,
    };
}

fn main() {
    println!("{}", White.bg(White).bold().paint("                 "));
    println!(
        "{}{}{}",
        White.bg(White).bold().paint(" "),
        Cyan.bg(White).bold().paint(" GUESSING GAME "),
        White.bg(White).bold().paint(" ")
    );
    println!("{}", White.bg(White).bold().paint("                 "));
    println!();

    loop {
        play_game();
        println!();

        if !ask_for_retry() {
            println!("Good bye");
            break;
        }

        println!("Let's try again!");
    }
}
