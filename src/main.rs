fn main() {
    let mut arr: Vec<i32>  = vec![1, 2, 3, 0];
    println!("{:?}", arr);
    let size = arr.len()-1;
    merge_sort(&mut arr, 0, size);
}

fn merge_sort(arr: &mut Vec<i32>, start: usize, end: usize) {
    if start < end {
        let mid = (start + end) / 2;
        merge_sort(arr, start, mid);
        merge_sort(arr, mid + 1, end);
        merge(arr, start, mid, end);
    }
}

fn merge(arr: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
    let mut temp: Vec<i32> = Vec::with_capacity(end - start + 1);
    let mut i: usize = start;
    let mut j: usize = mid + 1;

    while i <= mid && j <= end {
        if arr[i] <= arr[j] {
            temp.push(arr[i]);
            i += 1;
        } else {
            temp.push(arr[j]);
            j += 1;
        }
    }

    while i <= mid {
        temp.push(arr[i]);
        i += 1;
    }

    while j <= end {
        temp.push(arr[j]);
        j += 1;
    }

    println!("{:?}", temp);

    for i in start..=end {
        arr[i] = temp[i - start]
    }
}
