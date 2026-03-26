pub mod p88;

pub fn merge_sort(numbers: &mut [i32]) {
    if numbers.len() <= 1 {
        return;
    }

    let middle = numbers.len().midpoint(0);
    let end = numbers.len();
    merge_sort(&mut numbers[0..middle]);
    merge_sort(&mut numbers[middle..end]);

    merge(numbers);
}

// [5, 3]
pub fn merge(numbers: &mut [i32]) {
    if numbers.len() <= 1 {
        return;
    }
    let mut temp = Vec::with_capacity(numbers.len());

    let middle: usize = numbers.len() / 2;

    let mut l: usize = 0;
    let mut r: usize = middle;

    while l < middle && r < numbers.len() {
        match numbers[l].cmp(&numbers[r]) {
            std::cmp::Ordering::Greater => {
                temp.push(numbers[r]);
                r += 1;
            }
            _ => {
                temp.push(numbers[l]);
                l += 1;
            }
        }
    }

    for i in l..middle {
        temp.push(numbers[i]);
    }

    for i in r..numbers.len() {
        temp.push(numbers[i]);
    }

    numbers.copy_from_slice(&temp);
}

#[cfg(test)]
pub mod test {
    use crate::problems::sorting::merge::merge_sort;

    #[test]
    pub fn merge_sort_test() {
        let mut numbers: Vec<i32> = vec![5, 6, 7, 3, 1, 2, 4];
        merge_sort(&mut numbers);
        assert_eq!(numbers, vec![1i32, 2i32, 3i32, 4, 5, 6, 7]);
    }
}
