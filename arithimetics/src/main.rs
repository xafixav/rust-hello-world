fn main() {
    let x: i32 = 350;
    let y: u8 = 16;

    let sum: i32 = sum(x,y as i32);
    let sub: i32 = sub(x,y as i32);
    let mul: i32 = mul(x,y as i32);
    let div: i32 = div(x,y as i32);

    println!("this is the result of sum x+y: {}", sum);
    // this is the result of sum x+y: 366

    println!("this is the result of sub x+y: {}", sub);
    // this is the result of sub x+y: 334

    println!("this is the result of mul x+y: {}", mul);
    // this is the result of mul x+y: 5600
    println!("this is the result of div x+y: {}", div);
    // this is the result of div x+y: 21
}


fn sum(x: i32, y: i32) -> i32 {
    let sum: i32 = x as i32 + y as i32;
    return sum;
}

fn sub(x: i32, y: i32) -> i32 {
    let sub: i32 = x as i32 - y as i32;
    return sub;
}

fn mul(x: i32, y: i32) -> i32 {
    let mul: i32 = x as i32 * y as i32;
    return mul;
}

fn div(x: i32, y: i32) -> i32 {
    let div: i32 = x as i32 / y as i32;
    return div;
}
