fn main() {
    let x = 5;

    if x > 5 {
        println!("x is greater than 5");
    } else if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is less than 5");
    }

    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }
    println!("count: {}", count); //出力：count: 3


    let mut i = 0;
    while i < 5 {
        println!("{}", i);
        i += 1;
    }

    for i in 0..5 { // 0から4まで
        println!("{}", i);
    }
}
