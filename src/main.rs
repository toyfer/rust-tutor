fn main() {
    let s1 = String::from("hello");
    // この場合、mutを付けていないので、不変参照を指す
    
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    // この場合、mutを付けているので、可変参照を指す
    
    change(&mut s2); // s2の可変参照を渡す

    println!("s2 is now: {}", s2);
}

fn calculate_length(s: &String) -> usize { // 不変参照を受け取る関数
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}