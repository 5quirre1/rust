use std::collections::HashMap;

pub fn get_currencies() -> HashMap<&'static str, f64> {
    let mut currencies: HashMap<&str, f64> = HashMap::new();

    // coins
    currencies.insert("penny", 0.01);
    currencies.insert("nickel", 0.05);
    currencies.insert("dime", 0.10);
    currencies.insert("quarter", 0.25);
    currencies.insert("half dollar", 0.50);
    currencies.insert("dollar coin", 1.00);

    // bills
    currencies.insert("1 dollar", 1.00);
    currencies.insert("2 dollar", 2.00);
    currencies.insert("5 dollar", 5.00);
    currencies.insert("10 dollar", 10.00);
    currencies.insert("20 dollar", 20.00);
    currencies.insert("50 dollar", 50.00);
    currencies.insert("100 dollar", 100.00);
    currencies
}
