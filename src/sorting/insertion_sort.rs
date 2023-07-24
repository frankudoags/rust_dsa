/// Insertion sort algorithm is a simple algorithm that works the way we sort playing cards in our hands.
/// It considers one element at a time and inserts it at the right position in the sorted array.
/// It repeats until no input elements remain and the array is sorted.
/// 
/// # Example
/// ```
/// use sorting::insertion_sort::insertion_sort;
/// let mut arr = vec![6, 5, 4, 3, 2, 1];
/// insertion_sort(&mut arr);
/// assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
/// ```


// T: Ord means that the type T must implement the trait Ord which means that the type T must be comparable.
// Copy means that the type T must implement the trait Copy which means that the type T must be copyable.
// Copy is required because we are copying the value of the current element into a variable called curr.
// If we don't copy the value, we will get an error because we are trying to move the value out of the array.
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        let curr = arr[i];

        while j > 0 && curr < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = curr;
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
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &clone));
    }
    #[test]
    fn test_ascending() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        let clone = arr.clone();
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &clone));
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        let clone = arr.clone();
        insertion_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &clone));
    }
}