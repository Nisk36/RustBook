fn main(){
    for y in 1..10{
        for x in 1..10 {
            //print!マクロは改行を出力しない, 
            print!("{:3}," , x*y);
        }
        println!("");
    }
}