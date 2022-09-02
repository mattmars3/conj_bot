use crate::get_person;
use crate::get_conj::{ConjugationRule, ConjugationList};
use crate::webd;
use crate::runscript;


pub fn eval_arguments() {
    let args: Vec<_> = std::env::args().collect(); // get all arguements passed to app
    if args.len() < 2 {help();} 
    else {
        println!("ARG: {}", args.get(1).unwrap());
        if args.get(1).unwrap().eq("-h") {
            help();
        } else if args.get(1).unwrap().eq("-pt") {
            get_person::pronoun_test();
        } else if args.get(1).unwrap().eq("-r") {
            if args.len() != 4 {
                println!("ERROR: Must run with\n    -r username password");
            } else {
                // run the webdriver bot with credentials as parameters
                runscript::run();
            }
        } else if args.get(1).unwrap().eq("-ct") {
            conjugation_test();
        } else if args.get(1).unwrap().eq("-jt") {
            json_test()
        }
    }

}

fn json_test() {
    let c1 = ConjugationRule::prompt_new();
    let c2 = ConjugationRule::prompt_new();
    let clist: ConjugationList = ConjugationList::new();
    handle_json
}


fn conjugation_test() {
    println!("Running conjugation test");
    loop {
        println!("{}", get_conj::ConjugationRule::prompt_new());
    }
}



fn help() {
    println!("--- Command Line Options ---");
    println!("  -h - print this message");
    println!("  -r - run the bot with credentials that follow");
    println!("  -pt - test the pronoun system");
    println!("  -ct - test the conjugation system");
    println!("  -jt - test the json system");

}

