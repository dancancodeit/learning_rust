fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![1, 23, 32, 232];

    let result = largest(&number_list);

    let char_list = vec!['a', 's', 'f', 'a'];
    
    let result = largest(&char_list);
}
