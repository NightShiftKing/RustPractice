use std::io;

fn main(){
    let mut input = String::new();
    println!("How many lines of Tetris have you cleard? "); 

    io::stdin().read_line(&mut input).unwrap();
    //let response: i32 = input.parse().unwrap();
    let input: i32 = input.trim().parse().unwrap();

    if input < 100 {
        print!("Novice: 0-100 lines");
    }
    else {
        print!("noob"); 
    }

}