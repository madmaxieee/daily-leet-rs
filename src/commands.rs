use crate::languages::create_files;

pub fn daily() {
    println!("Fetching daily problem");
    fetch_data();
    create_files(
        "two-sum",
        "pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n    vec![]\n}",
        "assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);",
    )
    .unwrap();
}

pub fn problem() {
    println!("Fetching problem");
    fetch_data();
}

pub fn random() {
    println!("Fetching random problem");
    fetch_data();
}

fn fetch_data() {
    println!("Fetching data");
}
