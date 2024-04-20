pub fn merge_sort<T, F>(arr: &mut [T], compare: &F)
where
    T: Clone + Default,
    F: Fn(&T, &T) -> bool,
{
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = Vec::with_capacity(mid);
    let mut right = Vec::with_capacity(len - mid);

    for i in 0..mid {
        left.push(arr[i].clone());
    }
    for i in mid..len {
        right.push(arr[i].clone());
    }

    merge_sort(&mut left, compare);
    merge_sort(&mut right, compare);

    merge(arr, &left, &right, compare);
}

fn merge<T, F>(arr: &mut [T], left: &[T], right: &[T], compare: &F)
where
    F: Fn(&T, &T) -> bool,
    T: Clone + Default,
{
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if compare(&left[i], &right[j]) {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
