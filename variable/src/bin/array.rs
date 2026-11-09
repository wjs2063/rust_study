use std::io;




fn main(){

    let x = [1,2,3,4,5];

    let first = x[0];

    let second = x[1];

    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);


    let item = [1,2,3,4,5,6];

    let mut idx= String::new();

    io::stdin().read_line(&mut idx).expect("Failed to read line");

    // 입력 끝의 줄바꿈을 trim()으로 제거, 그리고 숫자로 변환
    let idx : usize = idx.trim().parse().expect("Not a number!");

    let element = item[idx];
    println!("The value of element is: {}", element);


}