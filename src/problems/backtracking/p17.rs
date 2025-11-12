pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        let map: HashMap<u8, Vec<u8>> = HashMap::from([
            (b'2', [b'a', b'b', b'c'].to_vec()),
            (b'3', [b'd', b'e', b'f'].to_vec()),
            (b'4', [b'g', b'h', b'i'].to_vec()),
            (b'5', [b'j', b'k', b'l'].to_vec()),
            (b'6', [b'm', b'n', b'o'].to_vec()),
            (b'7', [b'p', b'q', b'r', b's'].to_vec()),
            (b'8', [b't', b'u', b'v'].to_vec()),
            (b'9', [b'w', b'x', b'y', b'z'].to_vec()),
        ]);

        Self::backtrack(
            &mut result,
            digits.as_bytes(),
            &map,
            Vec::with_capacity(digits.len()),
        );

        result
    }

    pub fn backtrack(
        result: &mut Vec<String>,
        input: &[u8],
        map: &HashMap<u8, Vec<u8>>,
        mut current: Vec<u8>,
    ) {
        if input.is_empty() {
            result.push(current.into_iter().map(|x| x as char).collect::<String>());
            return;
        }

        let Some(letters) = map.get(&input[0]) else {
            panic!("uh oh");
        };

        for letter in letters.into_iter() {
            current.push(*letter);
            Self::backtrack(result, &input[1..], map, current.clone());
            current.pop();
        }
    }
}
