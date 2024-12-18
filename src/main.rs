fn main() {
    let mut vec = Vec::new(); // 空のベクターを初期化
    vec.push(1); // 要素を追加
    vec.push(2);
    vec.push(3);

    println!("ベクターの要素数: {}", vec.len()); // 出力: ベクターの要素数: 3
    println!("最初の要素: {}", vec[0]); // 出力: 最初の要素: 1

    // vec.push(4); // 要素を追加

    // for文でベクターの要素にアクセスする
    for x in &vec { //参照渡しなので元のvecの中身は変更されない
        println!("{}", x);
    }

    // 値渡しになるが、所有権がムーブするので、実行するとそれ以下のvecを用いたコードが実行できなくなる
    /*
    for x in vec { //値渡しなので元のvecの中身は変更されない
        println!("{}", x);
    }
    */

    let third: &i32 = &vec[2]; // vecの3番目の要素を参照
    println!("The third element is {}", third);

    match vec.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // vec.get(3) //範囲外を参照しようとするとNoneを返す
    
}
