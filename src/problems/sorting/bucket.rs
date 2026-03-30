pub mod p75;

/// # Runtime
/// - best case: O(n)
/// - worst case: O()
pub fn bucket_sort(numbers: &mut [i32]) {
    let mut count: [usize; 3usize] = [0usize; 3usize];
    for i in 0..numbers.len() {
        count[numbers[i] as usize] += 1;
    }

    let mut i: usize = 0;
    for x in 0..count.len() {
        let n = count[x];
        for _ in 0..n {
            numbers[i] = x as i32;
            i += 1;
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::problems::sorting::bucket::bucket_sort;

    #[test]
    pub fn bucket_sort_test() {
        let mut numbers = vec![2, 1, 2, 0, 0, 2];

        bucket_sort(&mut numbers);

        assert_eq!(numbers, vec![0, 0, 1, 2, 2, 2])
    }
}
