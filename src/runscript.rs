// manages the running bot
use crate::get_person::get_input;

pub fn run() {
    command_line_script();
}

fn command_line_script() {
    print!("Enter pronoun input: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let pi: String = get_input();
    print!("Enter Verb: ")
}
