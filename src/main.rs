// fn my_push(vector: &mut Vec<&String>, element: &String) {
//     vector.push(element);
//     // ^^^ error: rustc: lifetime may not live long enough
//     // argument requires that `'1` must outlive `'2`
//     // elements type is 1 and vecs type is 2.
//     // if lifetime params cannot be infered an error is thrown as by default lifetimes are ` '_ `
// }

// fn my_push<T>(vector: &mut Vec<T>, element: T) {
//     vector.push(element);
//     // this works as it tells the compiler that the args are the same
// }

fn my_push<'a, 'b>(vector: &'b mut Vec<&'a String>, element: &'a String) {
    vector.push(element);
}

fn main() {
    let my_string = String::from("FooBar");
    let mut my_vec = Vec::<&String>::new();
    // my_vec.push(&my_string);
    my_push(&mut my_vec, &my_string);
    // drop(my_string);
    println!("{:?}", my_vec);
}
