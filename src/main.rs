mod get_person;
mod cmd_line;
mod get_conj;
mod handle_json;
mod webd;
mod runscript;

fn main() {
    cmd_line::eval_arguments();
}

