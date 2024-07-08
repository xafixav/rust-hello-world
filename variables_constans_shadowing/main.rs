// fn main_variables() {
//     let x = 4;
//     println!("x = {}", x);
// }

// PS C:\code\rustmania\hello_world\variables_constants_shadowing> .\main.exe
// x = 4

// fn main_const() {
//     const NUMBER_FIVE: u32 = 5;
//     println!("NUMBER_FIVE = {}", NUMBER_FIVE);
// }


// PS C:\code\rustmania\hello_world\variables_constants_shadowing> .\main.exe     
// NUMBER_FIVE = 5


// fn main() {
//     let x = 4;
//     println!("x = {}", x);
//     let x = 18;
//     println!("x = {}", x);
// }


// PS C:\code\rustmania\hello_world\variables_constants_shadowing> .\main.exe     
// x = 4
// x = 18


// fn main() {
//     let x = 4;
//     println!("x = {}", x);

//     {
//         let x = x *4;
//         println!("x = {}", x);
//     }
//     let x = x * 2;
//     println!("x = {}", x);
// }

// PS C:\code\rustmania\hello_world\variables_constants_shadowing> .\main.exe     
// x = 4
// x = 16
// x = 8


fn main() {
    let mut x = 4;
    println!("x = {}", x);
    x = 4 * 2;
    println!("x = {}", x);
}


// PS C:\code\rustmania\hello_world\variables_constans_shadowing> .\main.exe     
// x = 4
// x = 8