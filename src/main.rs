use std::env;
use convert_case::{Case, Casing};

fn main() {

    let args: Vec<String> = env::args().collect();

    let help = "
    use format <type-to-convert-to> <text-to-convert>\n
    following types are available
    \n\tcapital,\n\tlower,\n\ttitle,\n\tsnake,\n\trandom\n
    for text spanning multiple words please use quotes
    ";

    if args.len() <= 2 {
        println!("not enough arguments supplied");
        println!("{}", help);
        std::process::exit(2);
    }

    let arg1 = &args[1];
    let arg2 = &args[2];

    

    match arg1.as_str(){
        "capital" => println!("{}", arg2.to_case(Case::Upper)),
        "lower" => println!("{}", arg2.to_case(Case::Lower)),
        "title" => println!("{}", arg2.to_case(Case::Title)),
        "snake" => println!("{}", arg2.to_case(Case::Snake)),
        "random" => println!("{}", arg2.to_case(Case::Random)),
        "help" => println!("{}", help),
        _ => println!("{}", help)
    }


}
