use std::io;
use rand::Rng;

fn main() {
    
    let mut target: i128 = 0; //수정이 가능할 초기화 int128로 늘려서 더 큰값을 저장할수있음, 당연히 적절한 크기 지정이 중요험

    while target != 100 { //==로 주는게 아니라 !=인 이유는 만족 못했을때 반복이 조건이라 그럼
        target += 1000*1000;
        println!("number is: {target}");
    }

    // for i in 1..10000000{
    //     if secret_number == 1{
    //         break;
    //     }
    //     println!("The secret number is: {secret_number}");
    // }    
    // println!("Please input your guess.");

    // let mut guess = String::new();

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    // println!("You guessed: {guess}");
}