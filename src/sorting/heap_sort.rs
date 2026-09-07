pub fn heapify<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();

    if n <= 0 {
        return;
    }

    for i in (0..n / 2).rev() {
        sift_down(arr, i, n);
    }
}

pub fn sift_down<T: PartialOrd>(arr: &mut [T], mut i: usize, n: usize) {
    loop {
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        let mut target = i;

        if left < n && arr[left] > arr[target] {
            target = left;
        }

        if right < n && arr[right] > arr[target] {
            target = right;
        }

        if target == i {
            break;
        }

        arr.swap(i, target);
        i = target;
    }
}

pub fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
    heapify(arr);

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        sift_down(arr, 0, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_sort_test() {
        let mut arr = [4, 999, 0, -7, 1];

        let expected = [-7, 0, 1, 4, 999];

        heap_sort(&mut arr);
        assert_eq!(arr, expected);
    }
}
