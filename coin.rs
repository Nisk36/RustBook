fn main(){
    //期待する金額
    let expected_price = 3950;
    for i500 in 0..11{
        for i100 in 0..4{
            for i50 in 0..11 {
                let total = i50 * 50 + i100 * 100 + i500 * 500;
                if expected_price == total{
                    println!("500円x{}+100円x{}+50円x{} = {}", i500, i100, i50, total);
                }
            }
        }
    }
}