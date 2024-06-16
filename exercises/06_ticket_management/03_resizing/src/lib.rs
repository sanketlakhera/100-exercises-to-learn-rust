#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // max capacity reached
        assert_eq!(v.capacity(), 2);

        v.push(3); // beyond capacity, needs to resize

        // Can you guess what the new capacity will be?
        // Beware that the standard library makes no guarantees about the
        // algorithm used to resize the vector, so this may change in the future.
        assert_eq!(v.capacity(), 4);
        /// In Rust, when a Vec needs to increase its capacity, it typically doubles
        /// the current capacity to amortize the cost of reallocations over many
        /// insertions. This is why you observe the capacity increasing to 4 when
        /// you push a third element into a Vec with an initial capacity of 2.
    }
}
