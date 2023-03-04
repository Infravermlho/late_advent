fn get_largest(array: &[i32]) -> &i32 {
    let mut largest = &array[0];

    for item in array {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    println!("Hello, world!");

    let foda = [10, 20, 10 , 1];
    let maior = get_largest(&foda);

    println!("{}", maior)
}
