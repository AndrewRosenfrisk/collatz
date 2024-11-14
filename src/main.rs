use std::io::{stdin, stdout, Write};

fn main() -> Result<(), std::io::Error> {
    println!("Enter an integer greater than 0");

    let mut num: u64 = get_input().unwrap();

    print!("{}", num);
    while num != 1 {
        if num % 2 == 0 {
            num = num / 2;
        } else {
            num = 3 * num + 1;
        }
        print!(", {}", num)
    }
    stdout().flush()?;
    println!("\n");
    Ok(())
}

fn get_input() -> Result<u64, std::io::Error> {
    let mut input = String::new();
    'input: loop {
        stdin().read_line(&mut input)?;

        if input.trim().parse::<u64>().unwrap_or(0) == 0 {
            println!("Invalid entry, please try again.");
            continue;
        } else {
            break 'input;
        }
    }
    Ok(input.trim().parse::<u64>().unwrap_or(0))
}
