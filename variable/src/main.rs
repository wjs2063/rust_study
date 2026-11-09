fn main() {
    /*
    일반적으로 rust 의 변수는 immutable
    */

    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6 ;
    println!("The value of x is: {}", x);


    println!("Hello, world!");

    let mut y = 7;
    println!("The value of y is: {}", y);
    y = 8;
    println!("The value of y is: {}", y);


    // 문자를 unsigned int 로 변환
    let guess: u32 = "42".parse().expect("Not a number!");

    



}
