use std::io::Read;


fn load_data(path: &str) -> Vec<i32> {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let numbers = contents.lines()
        .filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>();
    numbers
}

fn count_increments(numbers: Vec<i32>) -> i32 {
    let count = numbers.windows(2)
        .map(|slice| slice[1] - slice[0] > 0)
        .filter(|b| *b)
        .count();
    count as i32
}

fn count_sliding_window_increments(numbers: Vec<i32>) -> i32 {
    let count = numbers.windows(3)
        .map(|slice| slice.into_iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|slice| slice[1] - slice[0] > 0)
        .filter(|b| *b)
        .count();
    count as i32
}

fn main() {
    let numbers = load_data("data/input-part-1.txt");
    let n_increments = count_increments(numbers);
    println!("The number of increments is {}", n_increments);

    let numbers = load_data("data/input-part-1.txt");
    let n_windowed_increments = count_sliding_window_increments(numbers);
    println!("The number of windowed increments is {}", n_windowed_increments);
}
