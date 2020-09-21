use std::fs;
use std::env::args;

fn main() -> std::io::Result<()> {
    let args_list: Vec<String> = args().collect();
    let file_one = &args_list[1];
    let file_two = &args_list[2];
    let dist = file_two.clone() + &file_one.clone();
    println!("{}", dist);

    fs::rename(file_one, dist)?;
    Ok(())
}
