fn add_numbers(num_1: i32, num_2: i32) -> i32 {
    return num_1 + num_2;
}

fn main() {
    // let mut _name = "amaan nadeem";
    // _name = "hello";
    // let number = add_numbers(1, 1);
    // println!("{}", number);
    // println!("Hello, world!");
    // println!("{0} {0}", number);
    // println!("{:?}", _name);

    // Total match statement
    let mut total = add_numbers(5, 10);
    let mut free_shipping = false;

    if total > 50 {
        println!("{}", total);
        free_shipping = true;
    } else if total < 20 {
        println!("{}", total);
        free_shipping = false;
    } else {
        println!("{} {}", total, free_shipping);
        free_shipping = false;
    }
    match free_shipping {
        true => total = total + 5,
        false => total = total + 0,
    }

    // OR
    // total =  match free_shipping {
    //     true => total + 5,
    //     false =>  total + 0
    // }

    match total {
        0 => println!("match found"),
        1 => println!("match found"),
        _ => println!("No match found!"),
    }

    // array in rust
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{} {:?}", "Printing an Array: ", arr);

    // vectors in rust
    let vec_items_1 = vec![1, 2, 3, 4, 5];
    let mut vec_items_2 = Vec::new();
    vec_items_2.push(1);
    vec_items_2.push(2);
    vec_items_2.push(3);
    vec_items_2.push(4);
    vec_items_2.push(5);

    println!("{} {:?}", "Printing a Vector: ", vec_items_1);
    println!("{} {:?}", "Printing a Vector: ", vec_items_2);
}
