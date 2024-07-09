fn encrypt(text: &str, shift: i16) -> String{
    //'A'と'Z'の文字コードをi16型で得る
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    // 結果を代入する変数を用意
    let mut result = String::new();
    
    for ch in text.chars(){
        //一文字ずつ文字コードに変換
        let mut code = ch as i16;
        if code_a <= code && code <= code_z{
            //shift分ずらす
            code = (code - code_a + shift + 26) % 26 + code_a;
        }
        result.push((code as u8) as char);
    }
    return result;
}

fn main(){
    let enc = encrypt("I LOVE YOU.", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec)
}

// Rust では’A'が文字. "A"が文字列
// asが強制的な方変換
// Rustで変更不可能な文字列リテラルを表す型は &str, String型がある
