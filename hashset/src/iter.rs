pub struct Iter<'a, T> {
    pub(crate) bucket_iter: std::slice::Iter<'a, Vec<T>>,
    pub(crate) current_bucket: Option<std::slice::Iter<'a, T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
        loop {

			if let Some(ref mut bucket) = self.current_bucket {
				if let Some(item) = bucket.next() {
					return Some(item);
				}
			}

			match self.bucket_iter.next() {
				Some(bucket) => {
					self.current_bucket = Some(bucket.iter());
				},
				None => return None
			}

		}
    }
}
