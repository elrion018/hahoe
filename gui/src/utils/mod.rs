pub fn get_head<T>(vector: &Vec<T>) -> Option<&T> {
    if vector.is_empty() {
        None
    } else {
        Some(&vector[0])
    }
}
