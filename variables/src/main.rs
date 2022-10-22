// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let floored = 2 / 3; // Results in 0

//     // remainder
//     let remainder = 43 % 5;

//     println!(
//         "{} {} {} {} {} {}",
//         sum, difference, product, quotient, floored, remainder
//     );

//     let tup = (500, 6.4, 1);

//     let first = tup.0;
//     let (x, y, z) = tup;
//     println!("{} {} {}", x, y, z);

//     let array: [u32; 3] = [1, 2, 3];
//     let first = array[0];
//     let array = [1; 10]; // 1の要素を10個もつ
// }
// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x
//     };

//     let z = five();

//     println!("The value of y is: {y}");
//     println!("{z}");

//     let num = if z == 5 { 10 } else { 3 };
//     println!("{num}");

//     loop {
//         println!("loop !");
//         break;
//     }
// }

// fn five() -> i32 {
//     return 5;
// }

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    for number in (1..5).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
