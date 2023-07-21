/// Selection sort is a sorting algorithm that selects the smallest element from an unsorted list in each iteration
/// and places that element at the beginning of the unsorted list.
/// This effectively shrinks the unsorted list, till all elements are sorted.
/// 
/// # Example
/// ```
/// use sorting::selection_sort::selection_sort;
/// let mut arr = vec![6, 5, 4, 3, 2, 1];
/// selection_sort(&mut arr);
/// assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
/// ```

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let left = arr.len();
    // first loop
    for i in 0..left {
        //variable to store the index of the smallest element
        //set it to the current index
        let mut min = i;
        // second loop, used to find the smallest element, 
        // starting from the current index
        // if the current element is less than the smallest element
        // set the index of the smallest element to the current index
        for j in i + 1..left {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        // after finding the smallest element, swap it with the current element, and move on
        arr.swap(i, min);
    }
}

/// Illustration of selection sort
/// 
/// [6, 5, 4, 3, 2, 1]
/// after first iteration - [1, 5, 4, 3, 2, 6] -- 1 is the smallest element, so it is swapped with 6
/// after second iteration - [1, 2, 4, 3, 5, 6] -- 2 is the smallest element, so it is swapped with 5
/// after third iteration - [1, 2, 3, 4, 5, 6] -- 3 is the smallest element, so it is swapped with 4
/// after fourth iteration - [1, 2, 3, 4, 5, 6]
/// after fifth iteration - [1, 2, 3, 4, 5, 6]
/// after sixth iteration - [1, 2, 3, 4, 5, 6]
/// array is sorted!


#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::{is_sorted, have_same_elements};

    #[test]
    fn test_basic() {
        let mut arr = vec!["d", "a", "c", "b"];
        let clone = arr.clone();
        selection_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &clone));
    }
    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        let clone = arr.clone();
        selection_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &clone));
    }
    #[test]
    fn test_one_element() {
        let mut arr = vec![1];
        let clone = arr.clone();
        selection_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &clone));
    }
    #[test]
    fn test_pre_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        let clone = arr.clone();
        selection_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &clone));
    }
}