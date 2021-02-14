//n번째 피보나치 수
//1. 반복문
//2. 재귀함수

use::std::io;

fn main() {
    let number = loop {
        println!("Input the number.");

        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Fail to read line");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try again...");
                continue
            },
        };

        break number;
    };

    println!("{}th fibonacci number is {}.", number, fibonacci(number));

}

fn fibonacci(num:u32)->u32{
    let mut n1 = 1;
    let mut n2 = 1;

    if num<3 {
        return 1;
    }
    for _ in 3..num+1{
        let tmp = n1;
        n1 = n2;
        n2 = tmp+n1;
    }
    n2
}