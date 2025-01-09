fn main() {
    println!("{}", reverse_num(1523));
}

fn recursive_sum(n: i32 ) -> i32{
    if n < 1{
        return 0; 
    }
    else{
         n + recursive_sum(n-1)
    }
}

fn reverse_num(i: i32) -> i32{
    if i == 0{
        return 0;
    }
    else{
        (i % 10) * 10i32.pow(i.to_string().chars().count() as u32) + reverse_num(i / 10)
    }
}