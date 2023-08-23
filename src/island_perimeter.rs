pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;

    for row in 0..grid.len() {
        for item in 0..grid[row].len() {
            if grid[row][item] == 0 {
                continue;
            }

            // top
            if row as i32 - 1 < 0 || grid[row - 1][item] == 0 {
                res += 1;
            }

            // left
            if item as i32 - 1 < 0 || grid[row][item - 1] == 0 {
                res += 1;
            }

            // right
            if item + 1 == grid.len() || grid[row][item + 1] == 0 {
                res += 1
            }

            // bottom
            if row + 1 == grid[row].len() || grid[row + 1][item] == 0 {
                res += 1;
            }

            println!("row: {} item: {} res:{}", row, item, res);
        }
    }

    return res;
}

struct TestCase {
    grid: Vec<Vec<i32>>,
    expected: i32,
}

#[test]
fn test_island_perimeter() {
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
        assert_eq!(island_perimeter(case.grid), case.expected);
    }
}
