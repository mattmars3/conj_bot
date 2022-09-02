
// this is where all the conjugation happens
use serde::{Serialize, Deserialize};
use crate::get_person::get_input;
use std::fmt;


#[derive(Debug, Serialize, Deserialize)]
pub struct ConjugationList {
    set_name: String,
    conj_map: Vec<ConjugationRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConjugationRule {
    verb_name: String,
    conjugations: Vec<String>,
}

// The list of conjugations and name of verb
impl ConjugationRule {
    fn new(verb_name: String, conjs: Vec<String>) -> ConjugationRule {
        ConjugationRule { 
            verb_name,
            conjugations: conjs,
        }
    }

    pub fn prompt_new() -> ConjugationRule {
        print!("Enter verb name: ");
        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
        let verb_n: String = get_input();
        println!("Conjugate {}", verb_n);
        let mut conj_vec: Vec<String> = vec![];
        for i in 0..6 {
            std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
            conj_vec.push(get_input()); 
        }
        ConjugationRule::new(verb_n, conj_vec)
    }
}
impl fmt::Display for ConjugationRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Verb: {}\nYo: {}\nTu: {}\nUsted: {}\nNosotros: {}\nVosotros: {}\nUstedes: {}",
               self.verb_name, self.conjugations[0], self.conjugations[1], self.conjugations[2], 
               self.conjugations[3], self.conjugations[4], self.conjugations[5])
    }
}

// Person::FirstSingular => write!(f, "First Person Singular"),
/* 
 * Struct Conjugation Rule
 * Struct Set Rules
 * Json is a group of all of those
 *
 * */
