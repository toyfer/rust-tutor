rust
fn main() {
    let x = 5; // xは不変
    // x = 6; // エラー：不変の変数を変更しようとしました

    let mut y = 5; // yは可変
    y = 6; // OK
    println!("{}", y); // 出力: 6
}
