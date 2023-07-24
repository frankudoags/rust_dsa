pub mod sorting;


//Testing from lib file just to see if it works
#[cfg(test)]
mod tests {
    use super::sorting;
    #[test]
    fn bubble_sort() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        sorting::bubble_sort::bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }
}