pub fn merge_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    merge(arr, mid);
}

fn merge<T: PartialOrd>(arr: &mut [T], mid: usize) {
    let mut i = 0;
    let mut m = mid;

    while i < m && m < arr.len() {
        if arr[i] <= arr[m] {
            i += 1;
            continue;
        }

        let mut j = m + 1;
        while j < arr.len() && arr[j] < arr[i] {
            j += 1;
        }

        arr[i..j].rotate_left(m - i);

        let moved = j - m;

        i += moved;
        m = j;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sort_test() {
        let mut arr = [4, 999, 0, -7, 1];

        let expected = [-7, 0, 1, 4, 999];

        merge_sort(&mut arr);
        assert_eq!(arr, expected);
    }
}
