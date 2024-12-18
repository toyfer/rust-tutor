fn main() {
    let result = add(3, 4); // 関数を呼び出す
    println!("result: {}", result); //出力：result: 7
}

fn add(x: i32, y: i32) -> i32 { // 関数を定義する
    x + y
}
