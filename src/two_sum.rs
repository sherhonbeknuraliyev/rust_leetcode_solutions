use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        match num_map.get(&(target - nums[i])) {
            Some(v) => return vec![*v, i as i32],
            None => {
                num_map.insert(nums[i], i as i32);
            }
        }
    }

    return vec![];
}

struct TestCase {
    nums: Vec<i32>,
    target: i32,
    expected: Vec<i32>,
}

#[test]
fn test_two_sum() {
    let test_cases: Vec<TestCase> = vec![
        TestCase {
            nums: todo!(),
            target: todo!(),
            expected: todo!(),
        },
        TestCase {
            nums: todo!(),
            target: todo!(),
            expected: todo!(),
        },
    ];

    for case in test_cases {
        assert_eq!(two_sum(case.nums, case.target), case.expected);
    }
}
