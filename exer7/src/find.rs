pub fn find_elt<T: Eq>(values: &Vec<T>, elt: T) -> Option<usize> {
    // TODO
    for (index, value) in values.iter().enumerate() {
        if *value == elt {
            return Some(index);
        }
    }
    None
}
