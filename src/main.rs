use std::env;
use std::fs;


fn main(){
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let output = &args[2];
    match fs::rename(input,output){
    Ok(_) => {
        println!("File \'{}\' succesfully moved to \'{}\'",input,output);
    }
    Err(_) => {
        println!("Failed to move file \'{}\' to \'{}\'",input,output);
    }

    }
    // file_move(input,output);
    


}



// fn file_move(input: &str ,output: &str ) -> std::io::Result<()> {
//     fs::rename(input, target)?;
//     Ok(())
