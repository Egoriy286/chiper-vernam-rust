use std::fs::File;
use std::io::Read;
use rand::Rng;
fn main() {
    // Read a file in the local file system
    let mut data_file = File::open("data.txt").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    // Create an empty mutable Vec
    let mut key = Vec::new();

    for i in 0..file_content.len(){
        key.push(rand::thread_rng().gen_range(0,255));
        let num = u32::from(file_content.chars().nth(i).unwrap());
        let c= char::from_u32(num^key[i]);
        println!("Encode: {} => {:?}", num^key[i], c);
    }
    print!("Key=");
    for i in 0..file_content.len(){
        print!("{:?}",char::from_u32(key[i]));
    }
}
/*
fn main() {
    // Read a file in the local file system
    let mut data_file = File::open("data.txt").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let text = file_content.to_string();
    let mut key = Vec::new();
    //let mut encode:String="";
    for i in 0..text.len(){
        key.push(rand::thread_rng().gen_range(0,255));
        let num = u32::from(text.chars().nth(i).unwrap());
        let c= char::from_u32(num^key[i]);
        println!("File content: {} => {:?}", num^key[i], c);
    }
    print!("Key=");
    for i in 0..text.len(){
        print!("{:?}",char::from_u32(key[i]));
    }
    
}
*/
