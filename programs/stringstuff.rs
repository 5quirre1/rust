fn main() {
    let instructions = "step 1: turn the stove on and put the pan on\nstep 2: food in pan\nstep 3: eat food";
    let step_number = 2;
    let keyword = format!("step {}", step_number);

    if let Some(start) = instructions.find(&keyword) {
        let after = &instructions[start..];
        let next_newline = after.find('\n');

        let result = match next_newline {
            Some(index) => &after[..index],
            None => after,
        };

        println!("{}", result);
    } else {
        println!("no found");
    }
}
