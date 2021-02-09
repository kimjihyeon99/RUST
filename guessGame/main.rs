//프로그램이 1~100사이의 랜덤한 정수를 생성
//사용자가 숫자를 입력할 때마다 프로그램은 사용자가 입력한 수가 정답보다 큰지, 작은지 알려줌
//사용자가 정답을 입력하면 게임은 종료

//사용자 입력 받기 위한 라이브러리
use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Game starts!");
    //컴퓨터 랜덤넘버 생성
    let com_number =  rand::thread_rng().gen_range(1,101);
   // println!("The secret number is {}",com_number);

    loop {
        println!("Please input your guess.");
        //String을 new함수를 이용해 guess라는 이름을 가진 mutable 변수를 선언
        let mut guess = String::new();


        //&은 값을 참조하겠다는 의미.
        //expect 는 Result  반환!
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse()
        //     .expect("please type a number!");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&com_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}