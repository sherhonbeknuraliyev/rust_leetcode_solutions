// 482. License Key Formatting

// You are given a license key represented as a string s that consists of only alphanumeric characters and dashes.
// The string is separated into n + 1 groups by n dashes. You are also given an integer k.
// We want to reformat the string s such that each group contains exactly k characters, except for the first group, which could be shorter than k but still must contain at least one character. Furthermore, there must be a dash inserted between two groups, and you should convert all lowercase letters to uppercase.

// Return the reformatted license key.

pub fn license_key_formatting(s: String, k: i32) -> String {
    todo!()
}

struct TestCase {
    s: String,
    k: i32,
    expected: String,
}

#[test]
fn test_license_key_formatting() {
    let test_cases = vec![
        TestCase {
            s: "5F3Z-2e-9-w".to_string(),
            k: 4,
            expected: "5F3Z-2E9W".to_string(),
        },
        TestCase {
            s: "2-5g-3-J".to_string(),
            k: 2,
            expected: "2-5G-3J".to_string(),
        },
    ];

    for test in test_cases {
        assert_eq!(license_key_formatting(test.s, test.k), test.expected);
    }
}
