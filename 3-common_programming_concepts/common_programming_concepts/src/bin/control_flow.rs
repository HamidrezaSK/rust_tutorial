fn main() {
    // let condition = false;
    // let number = if condition { 5 } else { 6 };

    // println!("The value of number is: {number}");

    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {number}");

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");
    
    let array : [i32::5] = [1,2,3,4,5];

    for number in (1..array.len()).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}