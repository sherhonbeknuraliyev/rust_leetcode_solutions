mod island_perimeter;
mod license_key_f;
mod two_sum;
struct TestCase {
    grid: Vec<Vec<i32>>,
    expected: i32,
}
fn main() {
    let test_cases = vec![
        TestCase {
            grid: vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0],
            ],
            expected: 16,
        },
        TestCase {
            grid: vec![vec![1]],
            expected: 4,
        },
    ];

    for case in test_cases {
        println!("res:{}", island_perimeter::island_perimeter(case.grid))
    }
}
