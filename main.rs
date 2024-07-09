fn main(){
    let moondis = 384400.0;
    let car = 80.0;
    let train = 300.0;

    println!("車で月まで{}日", moondis / car / 24.0);
    println!("新幹線で月まで{}日", moondis / train / 24.0);
}