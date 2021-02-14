//화씨 변환 프로그램
//F = C * 1.8+32
//C = (F-32) /1.8

use::std::io;

fn main() {
    println!("[1] Convert F to C");
    println!("[2] Convert C to F");

    //옵션 선택
    let choice = loop{
        println!("input the choice");

        let mut choice = String::new();

        io::stdin().read_line(& mut choice)
            .expect("Fail to read line");

        let choice:u32= match choice.trim().parse() {
            Ok(num) => num,
            Err(_) =>{
                println!("input number 1 or 2");
                continue
            },
        };

        if choice>0 && choice<3 {
            break choice;
        }
    };
    //온도 입력
    let temperature = loop{
        println!("input the temperature");

        let mut temperature = String::new();

        io::stdin().read_line(& mut temperature)
            .expect("Fail to read line");

        let temperature:f64  =match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) =>{
                println!("with out unit");
                continue
            },
        };

        break temperature;
    };

    let temperature = if choice ==1{
        f_to_c(temperature)
    }else{
        c_to_f(temperature)
    };

    println!("result: {}", temperature);

}

fn f_to_c(temp:f64) ->f64{
    (temp-32.) / 1.8
}

fn c_to_f(temp:f64) ->f64{
    temp*1.8 + 32.
}
