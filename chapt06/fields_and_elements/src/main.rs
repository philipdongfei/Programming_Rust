fn main() {
    let mut vec = vec![8, 29, 1, -1, 22, 88];
    quicksort(&mut vec);
    println!("{:?}", vec);
}

fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let len = slice.len();
    let pivot = len - 1;
    let mut i = 0;
    let mut j = 0;

    while j < len - 1 {
        if slice[j] <= slice[pivot] {
            slice.swap(i, j);
            i += 1;
        }
        j += 1;
    }

    slice.swap(i, len - 1);

    i
}

fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return; // Nothing to sort.
    }

    // Partition the slice into two parts, front and back.
    let pivot_index = partition(slice);

    // Recursively sort the front half of `slice`.
    quicksort(&mut slice[.. pivot_index]);

    // And back half.
    quicksort(&mut slice[pivot_index + 1 ..]);
}
