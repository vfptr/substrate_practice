use substrate_practice::traffic_sigal::{LightsDuration, Signal};
use substrate_practice::shape::{Circle, Triangle, Square, self};

// Calculate the sum of u32 integers
fn sum(arr: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &i in arr {
        // check whether the result is overflow.
        if let Some(result) = sum.checked_add(i) {
            sum = result;
        } else {
            return None;
        }
    }
    Some(sum)
}


fn main(){
    // Question 1.
    println!("\nResult For Question 1:");
    let arr = vec![Signal::Red(10), Signal::Yellow(3), Signal::Green(60)];
    for i in arr {
       println!("{:?}, lights duration = {}", i, i.duration()) 
    }

    // Question 2.
    println!("\nResult For Question 2:");
    let numbers = [1, 2, 3, 4, 5];
    print!("{:?}", numbers);
    match sum(&numbers) {
        Some(result) => println!("sum result = {}", result),
        None => ()
    }
    
    // Question 3.
    println!("\nResult For Question 3:");
    let circle = Circle{radius:5.0};
    let triangle = Triangle{base: 10.1, height: 5.2};
    let square = Square{side: 1.735};
    shape::print_area(&circle);
    shape::print_area(&triangle);
    shape::print_area(&square);
}