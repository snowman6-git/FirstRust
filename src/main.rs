use std::io;

fn main() {
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)//이게 진짜 해당 인풋받기
        .expect("Failed to read line"); //실패시
 
    println!("문자 수: {}", text.chars().count()); // 5
    println!("바이트 수: {}", text.len()); // 15 (각 한글 문자가 3바이트)

    for i in 0..3{
        println!("{}", text[i]);
    }
}