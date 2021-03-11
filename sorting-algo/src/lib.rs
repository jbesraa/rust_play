pub trait Sorter {
    fn sort<T>(slice: &mut [T]) where T: Ord;
}

pub fn sort<T, S>(slice: &mut [T]) where T: Ord, S: Sorter {
    S::sort(slice)
}

// mode_bubblesort;

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut[T]) where T: Ord {
            slice.sort();
        }
    }
    #[test]
    fn std_works() {
        let mut items = vec![4,2,3,1];
        sort::<_, StdSorter>(&mut items);
        assert_eq!(items, [1,2,3,4]);
    }
}
