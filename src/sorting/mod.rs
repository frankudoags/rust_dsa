pub mod bubble_sort;
pub mod selection_sort;
pub mod insertion_sort;

// pub use self::bubble_sort::bubble_sort;
pub use self::selection_sort::selection_sort;
pub use self::insertion_sort::insertion_sort;

#[cfg(test)]
use std::cmp;

#[cfg(test)]
// Helper function to check if two slices have the same elements
pub fn have_same_elements<T>(a: &[T], b: &[T]) -> bool
where
    // T: cmp::PartialOrd,
    // If HashSet is used
    T: cmp::PartialOrd + cmp::Eq + std::hash::Hash,
{
    use std::collections::HashSet;

    match a.len() == b.len() {
        true => {
            // This is O(n^2) but performs better on smaller data sizes
            // b.iter().all(|item| a.contains(item))

            // This is O(n), performs well on larger data sizes
            let set_a: HashSet<&T> = a.iter().collect();
            let set_b: HashSet<&T> = b.iter().collect();
            set_a == set_b
        }
        false => false,
    }
}

#[cfg(test)]
// Helper function to check if a slice is sorted
pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: cmp::PartialOrd,
{
    arr.windows(2).all(|w| w[0] <= w[1])
}
