use rand::Rng;

// Bubbling sort, support input parameter which implement ParitalOrd trait
fn bubbling_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len(); 

    // This loop controls the number of rounds in the sorting process. 
    // In each round, the largest unsorted element is moved to the end.
    for round in 0..len {
        // This loop compares adjacent elements and swaps them 
        // In each round, the largest unsorted element is moved to the end of the unsorted portion.
        for index in 0..len - round - 1 {
            if arr[index] > arr[index + 1] {
                // If the current element is greater than the next element, swap the elements
                arr.swap(index, index + 1);
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    // Generate a random array length between 5 and 10
    let array_length = rng.gen_range(5..=10); 

    let mut arr = Vec::new();
    for _ in 0..array_length {
        // Generate a random integer between 1 and 1000
        arr.push(rng.gen_range(1..=1000)); 
    }
    println!("input arr: {:?}", arr); 
    bubbling_sort(&mut arr);
    println!("result after sort: {:?}", arr); 
}