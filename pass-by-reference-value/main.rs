/*
    Code example of passing vectors by value and by reference.
    Original code source inspired by Coursera Rust Fundamentals course.
    https://www.coursera.org/learn/rust-fundamentals

*/
fn pass_by_value_vector_int(mut vec: Vec<i32>, val: i32) -> Vec<i32> {
    vec.insert(0, val);
    vec.push(val);
    vec
}

fn pass_by_reference_vector_int(vec: &mut Vec<i32>, val: i32) {
    vec.insert(0, val);
    vec.push(val);
}

fn vector_and_value() {
    /* Example of passing a vector by value and by reference */
    println!("Example of passing a vector and integer by value and by reference");

    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    let val: i32 = 99;

    // Mutates vector in place. Vector is available after the function call.
    pass_by_reference_vector_int(&mut vector, val);
    println!("Vector modified in place: {:?}", &vector);

    // Mutates the vector inside the function as a new value.
    let returned_vector: Vec<i32> = pass_by_value_vector_int(vector, val);
    println!("Vector returned by function {:?}", returned_vector);
}

fn pass_by_value_two_vectors(mut v1: Vec<i32>, mut v2: Vec<i32>) -> Vec<i32> {
    /* Example of passing two vectors by value */
    v1.append(&mut v2);
    v1
}

fn pass_by_reference_two_vectors(v1: &mut Vec<i32>, v2: &mut Vec<i32>) {
    /* Example of passing two vectors by reference */
    v1.append(v2);
}

fn vector_and_vector() {
    /* Example of passing two vectors by value and by reference */
    println!("Example of passing two vectors by value and by reference");

    let mut vector: Vec<i32> = vec![5, 4, 3, 2, 1];
    let mut backwards_vector: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Mutates vector in place. Vector is available after the function call.
    pass_by_reference_two_vectors(&mut vector, &mut backwards_vector);
    println!(
        "Vector modified in place with another vector: {:?}",
        &vector
    );

    // Mutates the vector inside the function as a new value.
    let returned_vector: Vec<i32> = pass_by_value_two_vectors(vector, backwards_vector);
    println!(
        "Vector returned by function with another vector {:?}",
        returned_vector
    );
}

fn main() {
    vector_and_value();
    vector_and_vector();
}
