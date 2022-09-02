use crate::get_person;
use crate::get_conj;
use crate::webd;


pub fn eval_arguments() {
    let args: Vec<_> = std::env::args().collect(); // get all arguements passed to app
    println!("ARG: {}", args[1]);
    if args.len() < 2 {help();} 
    else {
        if args[1].eq("-h") {
            help();
        } else if args[1].eq("-pt") {
            get_person::pronoun_test();
        } else if args[1].eq("-r") {
            if args.len() != 4 {
                println!("ERROR: Must run with\n    -r username password")
            } else {
                // run the webdriver thing with credentials as parameters
            }
        } else if args[1].eq("-ct") {
            println!("Running conjugation test");
            loop {
                println!("{}", get_conj::ConjugationRule::prompt_new());
            }
        }
    }

}

fn help() {
    println!("--- Command Line Options ---");
    println!("  -h - print this message");
    println!("  -r - run the bot with credentials that follow");
    println!("  -pt - test the pronoun system");
    println!("  -pc - test the conjugation system")
}

