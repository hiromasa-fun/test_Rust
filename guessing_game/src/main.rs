//randを利用する宣言
extern crate rand;

//ライブラリ
use std::io;
use rand::Rng;

//エントリーポイント
fn main() {

   //数字を当てて見て！
    println!("Guess the number!");

    //乱数生成
    let select_number = rand::thread_rng().gen_range(1, 101);

    //乱数確認(debug用)
    println!("The select number is: {}", select_number);

    //予想値を入力してください
    println!("Please input your guess.");

    let mut guess = String::new();

    //行の読み取りに失敗しました
    io::stdin().read_line(&mut guess)
    	.expect("Failed to read line");

    //あなたの予想値:{}
    println!("You guessed: {}", guess);
}
