#[cfg(test)]
mod tests {
    use fs_lib::dir::*;
    use fs_lib::fl::*;

    #[test]
    fn dir_should_store_file() {
        let mut new_dir = Directory::new("dir");
        let new_file = File::new("file", "txt", 0);

        new_dir.store(new_file.clone());

        assert_eq!(new_dir.files().borrow().get(0), new_file);
    }
}