fn main(){
    // letで宣言した変数はimmutableな変数になる
    // 書き換え可能な変数は let mut で宣言. （わざわざ値が可変であることを明示する必要って覚えるといいかも)
    let mut a = 1;
    let mut b = 1;
    println!("{}", a);
    println!("{}", b);
    for _ in 0..30 {
        println!("{}", a+b);
        let tmp = a;
        a = b;
        b = tmp + a;
    }
}