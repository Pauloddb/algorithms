pub fn quick_sort<T: PartialOrd + Clone>(arr: &mut [T], start: usize, end: usize) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = arr[(start + end) / 2].clone(); // meio
    let mut l = start;
    let mut r = end;

    while l <= r {
        while arr[l] < pivot {
            l += 1;
        }
        while arr[r] > pivot {
            r -= 1;
        }

        if l <= r {
            arr.swap(l, r);
            l += 1;
            if r > 0 {
                r -= 1;
            }
        }
    }

    if start < r {
        quick_sort(arr, start, r);
    }
    if l < end {
        quick_sort(arr, l, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = [4, 999, 0, -7, 1];
        let len = arr.len();

        let expected = [-7, 0, 1, 4, 999];

        quick_sort(&mut arr, 0, len - 1);
        assert_eq!(arr, expected);
    }
}
