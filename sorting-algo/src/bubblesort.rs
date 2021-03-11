use super::Sorter;

pub struct Bubblesort;

impl Sorter for Bubblesort {
	fn sort<T>(slice: &mut [T]) where T: Ord {
		let mut swapped = true;
		while swapped {
			swapped = false;
			loop {
				let mut swapped = false;
				for i in 0..slice.len() {
					if i == slice.len() { continue; }
					if slice[i] > slice[i+1] {
						slice.swap(i, i+1);
						swapped = true;
					}
				}
			}
		}
	}
}