/// Bubble sort is a simple sorting algorithm that repeatedly steps through the list,
///  compares adjacent elements and swaps them if they are in the wrong order.
/// The pass through the list is repeated until the list is sorted.
/// The algorithm, which is a comparison sort, is named for the way smaller or larger elements
/// "bubble" to the top of the list.
/// 
/// # Example
/// ```
/// use sorting::bubble_sort::bubble_sort;
/// let mut arr = vec![6, 5, 4, 3, 2, 1];
/// bubble_sort(&mut arr);
/// assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);

// T: Ord means that the type T must implement the trait Ord 
// which means that the type T must be comparable.
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    //check if array is empty
    if arr.is_empty() {
        return;
    }
    //keep track of whether or not we swapped on the previous pass
    let mut swapped = false;
    // get the length of the array
    let mut n = arr.len();

    while !swapped {
        swapped = true;
        // iterate through the array
        for i in 0..n - 1 {
            // if the current element is less than the previous element
            if arr[i] > arr[i + 1] {
                // swap the elements
                arr.swap(i, i + 1);
                // set swapped to true
                swapped = false;
            }
        }
        // decrement n
        n -= 1;
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::{is_sorted, have_same_elements};

    #[test]
    fn test_descending() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        let clone = arr.clone();
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &clone));
    }
    #[test]
    fn test_ascending() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        let clone = arr.clone();
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &clone));
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        let clone = arr.clone();
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &clone));
    }
}