use std::io;

fn main() {
    println!("Guess the number!"); //출력

    println!("Please input your guess.");

    let mut guess = String::new(); //문자열이 들어갈 공간 할당 
    io::stdin()
        .read_line(&mut guess)//이게 진짜 해당 인풋받기
        .expect("Failed to read line"); //실패시

    guess = "AAA".to_string(); //mut(가변가능) 으로 선언됀 guess를 수정할 수 있음, 기본 따옴표는 &str이 되니까 .to_string()함수가 필요함
    println!("You guessed: {guess}");
}