/* create a file for a certain tense and name it whatever the conjuguemos is named
    return a hashmap of it
    have a regular rule
    Search rules
        search for specific word
        search for rule
            -er means ends in er
            .ui means contains ui
    
        search for end conjuguation
     
    
*/

use serde::{Serialize, Deserialize};
use crate::get_conj::{ConjugationList};
use serde_json;
use std::{fs::File, io::Write};


// converts map to json
fn serialize(verb_map: ConjugationList) -> String {
    let json_str = serde_json::to_string(&verb_map).unwrap();
    json_str
}

// converts String to ConjugationList
fn deserialize(file_content: String) -> ConjugationList {
    let conjlist: ConjugationList = match serde_json::from_str(&file_content) {
        Ok(cl) => cl,
        Err(_) => {ConjugationList::unit()}
    };
    conjlist
}

// writes String to json file
fn write_file(json_str: String, filename: String) {
    // create file if file doesn't exist
    File::create(filename.clone());
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("Error opening file after it was supposed to be created"),
    };
    write!(file, "{}", json_str);
}

// reads JSON file and returns String
fn read_file(filename: String) -> String {
    let file_path = format!("../conjugation_sets/{}", filename);
    let json_str: String = std::fs::read_to_string(file_path.as_str()).expect("Error in read_file function");
    json_str
    
}

