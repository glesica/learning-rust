fn main() {

    // Iterators

    let nums = vec![1, 2, 3, 4, 5];
    for i in &nums {
        println!("nums (reference) -> {}", *i);
    }
    for i in nums {
        println!("nums (moved) -> {}", i);
    }

    let mut range = 0..10;
    loop {
        match range.next() {
            Some(x) => {
                println!("range -> {}", x);
            },
            None => {
                break;
            }
        }
    }

    for i in 0..10 {
        println!("0..10 -> {}", i);
    }

    // Consumers

    let one_to_ten: Vec<_> = (1..11).collect();
    for i in &one_to_ten {
        println!("one_to_ten -> {}", *i);
    }

    let one_to_twenty = (1..21).collect::<Vec<_>>();
    for i in &one_to_twenty {
        println!("one_to_twenty -> {}", *i);
    }

    let even = (1..11).find(|x| *x % 2 == 0);
    match even {
        Some(x) => {
            println!("Got a number: {}", x);
        },
        None => println!("No numbers :(")
    }

    let some_nums = [1, 2, 3, 4, 5, 6, 7];
    let mut evens = some_nums.iter().filter(|&x| *x % 2 == 0);
    loop {
        match evens.next() {
            Some(x) => println!("evens -> {}", x),
            None => break
        }
    }

    let sum_to_ten = (1..11).fold(0, |sum, x| sum + x);
    println!("sum_to_ten = {}", sum_to_ten);
}
