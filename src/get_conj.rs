
// this is where all the conjugation happens
use serde::{Serialize, Deserialize};
use crate::get_person::{get_input, Person};
use std::fmt;


#[derive(Debug, Serialize, Deserialize)]
pub struct ConjugationList {
    set_name: String,
    conj_list: Vec<ConjugationRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConjugationRule {
    verb_name: String,
    conjugations: Vec<String>,
}

impl ConjugationList {
    pub fn unit() -> ConjugationList {
        let v: Vec<ConjugationRule> = vec![];
        ConjugationList {
            set_name: String::from("Test"),
            conj_list: v,
        }
    }

    pub fn new(set_name: String, conj_list: Vec<ConjugationRule>) -> ConjugationList {
        ConjugationList {
            set_name,
            conj_list,
        }
    }


    // conjugate
    pub fn conjugate(self, verb_infinitive: String, person: Person) -> String {
        // search for exact match         
        for conj_rule in self.conj_list {
            if conj_rule.verb_name.eq(verb_infinitive.as_str()) {
                // conjugate the verb
            }
        }

        // search for ending
        for conj_rule in self.conj_list {
            // does verb ending match?
            let verb_len = conj_rule.verb_name.len();
            let verb_stem: &str = &conj_rule.verb_infinitive[0..verb_len-3];
            let verb_ending
            // if yes then make a new conjugation rule


            if conj_rule.verb_name.eq(verb_ending) {

                let conjv: Vec<String> = conj_rule.conjugations.into_iter().map().collect();
                ConjugationRule {
                    verb_name: verb_infinitive,
                    conjugations: 
                }
            }

        }
        String::from("");
    }
}

// The list of conjugations and name of verb
impl ConjugationRule {
    pub fn new(verb_name: String, conjs: Vec<String>) -> ConjugationRule {
        ConjugationRule { 
            verb_name,
            conjugations: conjs,
        }
    }

    //
    pub fn conjugate(self, person: Person) -> String {
        // this function may cause a problem because it might move an owned value so therefore it
        // is not valid after its use
        match person {
            FirstSingular => *self.conjugations.get(0).unwrap(),
            SecondSingular => *self.conjugations.get(1).unwrap(),
            ThirdSingular => *self.conjugations.get(2).unwrap(),
            FirstPlural => *self.conjugations.get(3).unwrap(),
            SecondPlural => *self.conjugations.get(4).unwrap(),
            ThirdPlural => *self.conjugations.get(5).unwrap(),
        }
    }

    // for use only with a conjugation rule and NOT an actual conjugated verb
    pub fn conjugate_with_stem(self, person: Person, verb_stem: String) -> String {
        match person {
            FirstSingular => verb_stem.push_str(&self.conjugations.get(0).unwrap()),
            SecondSingular => verb_stem.push_str(&self.conjugations.get(1).unwrap()),
            ThirdSingular => verb_stem.push_str(&self.conjugations.get(2).unwrap()),
            FirstPlural => verb_stem.push_str(&self.conjugations.get(3).unwrap()),
            SecondPlural => verb_stem.push_str(&self.conjugations.get(4).unwrap()),
            ThirdPlural => verb_stem.push_str(&self.conjugations.get(5).unwrap()),
            _ => String::from("Unknown Person in conjugate with stem function"),
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
