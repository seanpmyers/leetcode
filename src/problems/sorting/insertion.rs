pub fn insertion_sort(data: &mut Vec<i32>) {
    for i in 1..data.len() {
        for j in (1..=i).rev() {
            if data[j - 1] <= data[j] {
                break;
            }
            data.swap(j - 1, j);
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::problems::sorting::insertion::insertion_sort;

    #[test]
    pub fn first_test() {
        let mut data: Vec<i32> = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }
}
