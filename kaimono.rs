fn main(){
    let pc = 98000.0;
    let a_fee = 1200.0;
    let a_rate = 0.8;
    let b_fee = 0.0;
    let b_rate = 0.9;
    //変数に値を代入しなかったらこんな感じで出力してもエラー出ない
    //なるべくイミュータブル推奨
    println!("A社 {}円", pc*a_rate + a_fee);
    println!("B社 {}円", pc*b_rate + b_fee);
}