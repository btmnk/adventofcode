pub struct FilePath {
    pub path: String,
}

impl FilePath {
    pub fn move_into(&mut self, dirname: &str) {
        if self.path == "/" {
            self.path.push_str(dirname);
            return;
        }

        self.path.push_str(&format!("/{}", dirname))
    }

    pub fn move_out(&mut self) {
        self.path = self.path.rsplit_once("/").unwrap().0.to_string();
    }
}
