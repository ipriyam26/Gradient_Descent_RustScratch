use core::num;

use rand::{Rng};

fn main()  {


    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u16> = Vec::new();
    numbers.push(rng.gen_range(0..128));
    for _ in 0..100 {
        let number: u16 = rng.gen_range(0..128);
        numbers.push(number);
    }
    let m = 2;
    let c = 7;

    let y:Vec<u16> = numbers.iter().map(
        |v| v*m + c
    ).collect();


    


    
}
