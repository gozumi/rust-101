fn main() {
    let list1 = [2, 133, 34, 23, 5443, 23, 43, 4, 4, 2];
    let list2 = [2133, 3423, 5, 44, 3, 23, 43, 4, 4, 2];
    println!("The largest number is {}", find_largest(&list1));
    println!("The largest number is {}", find_largest(&list2));
}

fn find_largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
