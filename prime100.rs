// Rustで素数を100個生成

fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if n % i == 0{
            return false;
        }
    }
    return true;
}

fn get_primes(primes: &mut[usize; 100]){ //&が値の参照、mut は値がmutであることを示し, [usize; 100]は要素数が100のusize型の配列
    let mut i = 0;
    let mut count = 0;
    while count < 100{
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main(){
    let mut primes = [0; 100];//要素数100の配列を0で初期化
    get_primes(&mut primes);//可変参照な値を引数に用いるとき,呼び出し側もmutであることを指定する必要がある.
    println!("{:?}", primes);
}