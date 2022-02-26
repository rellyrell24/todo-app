pub struct Path {
    pub prefix: String
}

impl Path {
    pub fn define(&self, following_path: String) -> String {
        // `to_owned`: allocated buffer for string literal
        return self.prefix.to_owned() + &following_path
    }
}