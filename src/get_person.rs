// This creates functionality to determine what person a pronoun input is 
use std::fmt;


#[derive(PartialEq, Debug)]
pub enum Person {
    FirstSingular,
    SecondSingular,
    ThirdSingular,
    FirstPlural,
    SecondPlural,
    ThirdPlural,
    Unknown
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Person::FirstSingular => write!(f, "First Person Singular"),
            Person::SecondSingular => write!(f, "Second Person Singular"),
            Person::ThirdSingular => write!(f, "Third Person Singular"),
            Person::FirstPlural => write!(f, "First Person Plural"),
            Person::SecondPlural => write!(f, "Second Person Plural"),
            Person::ThirdPlural => write!(f, "Third Person Plural"),
            Person::Unknown => write!(f, "Unknown"),
        }
    } 
}

// gets the tense given the pronoun input
fn get_person(pronoun_input: String) -> Person {
    let s: String = pronoun_input.clone().to_lowercase();
    if s.contains(" y ") {
        if s.contains("yo") {
            Person::FirstPlural
        } else if s.contains("tú") {
            Person::SecondPlural 
        } else { 
            Person::ThirdPlural
        }
    } else {
        if s.eq("yo") {
            Person::FirstSingular
        } else if s.eq("tú") {
            Person::SecondSingular
        }  else if s.eq("nosotros") {
            Person::FirstPlural
        } else if s.eq("vosotros") {
            Person::SecondPlural
        } else if s.eq("ustedes") {
            Person::ThirdPlural
        } else {
            Person::ThirdSingular
        }
    }
}

pub fn pronoun_test() {
    loop {
        println!("{}", get_person(get_input()));
    }
}
pub fn get_input() -> String {
    let mut s: String = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Error getting line");
    s.trim().to_string()
}


