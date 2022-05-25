use std::io::Write;

use monad::{io::IO, applicative::Applicative, mdo, monad::Monad};

fn get_line() -> IO<String> {
    let mut str = String::new();
    let _ = std::io::stdin().read_line(&mut str).expect("something went wrong");
    IO::pure(str)
}

fn print(s: String) -> IO<()> {
    println!("{}", s);
    std::io::stdout().flush().expect("something went wrong");
    IO::pure(())
}

fn main() {
    let _ = mdo! {
        let message = "Enter your name: ".to_string();
        print(message);
        name <= get_line();
        if name.len() > 10 {
            print("Your name is too long!".to_string())
        } else {
            print("Hello, ".to_string() + &name)
        }
    };
}
