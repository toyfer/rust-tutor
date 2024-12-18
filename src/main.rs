fn main() {
    // それぞれの要素に対して、異なる型の定義が可能
    let tup: (i32, f64 ,u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: i32 = tup.0;
    let y: f64 = tup.1;
    let z: u8 = tup.2;
    println!("x is {}, y is {}, and z is {}", x,y,z);
}