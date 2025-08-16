mod currency;

use inquire::{MultiSelect, Text};
use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    let currencies = currency::get_currencies();
    let currency_names: Vec<&str> = currencies.keys().cloned().collect();

    let selected_currencies =
        match MultiSelect::new("which currencies do you have?", currency_names.clone()).prompt() {
            Ok(selected) => selected,
            Err(_) => {
                println!("error occurred while selecting currencies,, please try again.");
                return;
            }
        };

    if selected_currencies.is_empty() {
        println!("no currencies selected. exiting.");
        return;
    }

    let mut total = 0.0;
    let mut wallet: HashMap<String, f64> = HashMap::new();

    for currency_name in &selected_currencies {
        let quantity = loop {
            let input = match Text::new(&format!("how many {} do you have?", currency_name))
                .with_help_message("enter a number (or 0 if none)")
                .prompt()
            {
                Ok(input) => input,
                Err(_) => {
                    println!("error reading input... please try again.");
                    continue;
                }
            };

            match input.trim().parse::<f64>() {
                Ok(num) if num >= 0.0 => break num,
                Ok(_) => println!("please enter a positive number."),
                Err(_) => println!("invalid number format. please try again."),
            };
        };

        let value = currencies.get(&currency_name[..]).unwrap_or(&0.0);
        let amount = quantity * value;
        wallet.insert(currency_name.to_string(), amount);
        total += amount;
    }

    println!("\nyour wallet:");
    for (name, amount) in &wallet {
        println!("{}: ${:.2}", name, amount);
    }

    println!("\ntotal money: ${:.2}", total);

    print!("\n\npress enter to exit..");
    io::stdout().flush().unwrap();
    let mut d = String::new();
    io::stdin()
        .read_line(&mut d)
        .expect("failed to read line.,.,");
}
