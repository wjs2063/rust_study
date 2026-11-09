


fn main(){
    let mut x: u32 = 0;

    let result = loop {
        x += 1;
        if (x + 1) % 3 == 0 {break x * x } else{continue}
    };

    println!("The result is {}", result);

}