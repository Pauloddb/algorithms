pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 1..(n - i) {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut arr = [4, 999, 0, -7, 1];

        let expected = [-7, 0, 1, 4, 999];

        bubble_sort(&mut arr);
        assert_eq!(arr, expected);
    }
}
