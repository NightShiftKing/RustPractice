fn reverse_num(i: i32) -> i32{
    if i == 0{
        return 0;
    }
    else{
        (i % 10) * 10i32.pow(i.to_string().chars().count() as u32) + reverse_num(i / 10)
    }
}


fn largest_digit(num: i32) -> i32{
    if num == 0{
        return 0;
    }
    else {
        if num % 10 > (num / 10) % 10{
            num % 10
        }
        else {
            largest_digit(num / 10)
        }

    }
}


