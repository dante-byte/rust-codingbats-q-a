use crate::warmup1::sleepIn;

mod warmup1;

fn main() {
    println!("Hello, world!");

    println!("{}",sleepIn(false,false));

    warmup1::monkeyTrouble(false,true);


    let m = 2;
    let n  = 3;

    let x = i32::abs(m - n);

    println!("{}",x);








}
