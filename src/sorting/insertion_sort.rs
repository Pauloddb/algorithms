pub fn inserton_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut i = 0;

    while i < arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            arr.swap(i, i + 1);

            let next = i;

            while i > 0 && arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);

                i -= 1;
            }
            i = next;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = [4, 999, 0, -7, 1];

        let expected = [-7, 0, 1, 4, 999];

        inserton_sort(&mut arr);
        assert_eq!(arr, expected);
    }
}
