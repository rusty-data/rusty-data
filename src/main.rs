use std::env;
use std::fs;


fn main(){
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let output = &args[2];
    file_move(input,output);
    


}



fn file_move(input:&String,output:&String){
    let clone_in = input.clone();
    let heap: Vec<&str> = clone_in.split("/").collect();
    println!("{:?}", heap);

    let index = heap.len() - 1;
    let part = &heap[index];
    println!("{}", part);

    let target = output.clone() + "/" + part.clone();

    fs::rename(input, target).expect("failed to move");
    
    


}

