mod question; // https://github.com/5quirre1/that-one-question-module/blob/main/rust/src/question.rs (js download ts and put it in the same thing as this)

use crate::question::question;

fn main() {
    let name: String = question("wat is ur name???", "put a name smh");
    match name.to_lowercase().as_str() {
        "greg" => println!("wow WOW greg"),
        "nomaakip" => println!("hi nomaakop wow"),
        "names" => println!("names ahahahah na ewas taken f        ffffff"),
        _ => println!("hi {}!!!", name),
    }
}
