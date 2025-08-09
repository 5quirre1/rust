/*
Cargo.toml:
----------------------------
    [package]
    name = "name"
    edition = "2021"
    [dependencies]
    battery = "0.7"
----------------------------
*/

use battery::{Manager};

fn main() -> Result<(), battery::Error> {
    let manager = Manager::new()?;

    for (i, maybe_batt) in manager.batteries()?.enumerate() {
        let batt = maybe_batt?;
        println!("battery #{}:", i);
        println!("  vendor:       {:?}", batt.vendor());
        println!("  percentage:   {:.2}%", batt.state_of_charge().value * 100.0);
        println!("  state:        {:?}", batt.state());
    }

    Ok(())
}
