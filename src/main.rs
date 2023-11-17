use std::fs::File;
use std::io::Read;
use rand::Rng;
//fn main() {
//    // Read a file in the local file system
//    let mut data_file = File::open("data.txt").unwrap();
//
//    // Create an empty mutable string
//    let mut file_content = String::new();
//
//    // Copy contents of file to a mutable string
//    data_file.read_to_string(&mut file_content).unwrap();
//    
//    let text = file_content.to_string();
//    let mut key = [0, 0, 0, 0, 0, 0, 0, 0];
//    //let mut encode:String="";
//    for i in 0..text.len(){
//        key[i]=rand::thread_rng().gen_range(0,255);
//        let num = u32::from(text.chars().nth(i).unwrap());
//        //encode.push();
//        let c= char::from_u32(num^key[i]);
//        println!("File content: {} => {:?}", num^key[i], c);
//    }
//    print!("Key=");
//    for i in 0..text.len(){
//        print!("{:?}",char::from_u32(key[i]));
//    }
//    
//}

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
    
    let text = file_content.to_string();
    let mut key = [0, 0, 0, 0, 0, 0, 0, 0];
    //let mut encode:String="";
    for i in 0..text.len(){
        key[i]=rand::thread_rng().gen_range(0,255);
        let num = u32::from(text.chars().nth(i).unwrap());
        //encode.push();
        let c= char::from_u32(num^key[i]);
        println!("File content: {} => {:?}", num^key[i], c);
    }
    print!("Key=");
    for i in 0..text.len(){
        print!("{:?}",char::from_u32(key[i]));
    }
    
}

fn xor(s: Vec<u8>, key: &[u8]) -> Vec<u8> {
    let mut b = key.iter().cycle();
    s.into_iter().map(|x| x ^ b.next().unwrap()).collect()
}