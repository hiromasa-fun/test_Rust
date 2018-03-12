//入力を取得するライブラリ
use std::io;

//エントリーポイント
fn main() {

   //数字を当てて見て！
    println!("Guess the number!");

    //予想値を入力してください
    println!("Please input your guess.");

    let mut guess = String::new();

    //行の読み取りに失敗しました
    io::stdin().read_line(&mut guess)
    	.expect("Failed to read line");

    //あなたの予想値:{}
    println!("You guessed: {}", guess);
}
