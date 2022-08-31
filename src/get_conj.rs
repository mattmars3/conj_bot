// This creates functionality to determine what 

#[derive(Debug)]
enum Person {
    FirstSingular,
    SecondSingular,
    ThirdSingular,
    FirstPlural,
    SecondPlural,
    ThirdPlural,
    Unknown
}

pub fn run() {
    let x: Vec<&str> = vec!["yo", "yo y john", "usted"];
    let f: Vec<Person> = x.into_iter().map(|x| get_person(x)).collect::<Vec<Person>>();
    f.into_iter().for_each(|x| print!("{:?} ", x));
}

// gets the tense given the pronoun input
fn get_person(pronoun_input: &str) -> Person {
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

