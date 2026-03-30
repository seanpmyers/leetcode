pub fn quick_sort(numbers: &mut [i32]) {
    if numbers.len() <= 1 {
        return;
    }

    let mut l: usize = 0;
    let pivot: usize = numbers.len() - 1;
    let length: usize = numbers.len();

    for i in 0..=pivot {
        if numbers[i] <= numbers[pivot] {
            numbers.swap(i, l);
            l += 1;
        }
    }

    numbers.swap(pivot, l);

    quick_sort(&mut numbers[0..(l - 1)]);
    quick_sort(&mut numbers[(l + 1)..length]);
}

#[cfg(test)]
pub mod test {
    use crate::problems::sorting::quick::quick_sort;

    #[test]
    pub fn sorted_test() {
        let mut numbers: Vec<i32> = vec![6, 2, 4, 1, 3];
        quick_sort(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, 4, 6]);
    }
}
