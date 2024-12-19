use std::io;
use rand::Rng;

fn main() {
    let mut target = 0; //수정이 가능할 초기화
    while target != 100 { //==로 주는게 아니라 !=인 이유는 만족 못했을때 반복이 조건이라 그럼
        target = rand::thread_rng().gen_range(1..=100); //1에서 100까지 가능
        println!("number is: {target}");
    }
    for i in 1..10000000{
        if secret_number == 1{
            break;
        }
    }
}