#[cfg(test)]
mod tests {
    use fs_lib::directory::*;
    use fs_lib::file::*;

    #[test]
    fn dir_should_store_file() {
        let mut new_dir = Directory::new("dir");
        let new_file = File::new("file", "txt", 0);

        new_dir.store(new_file.clone());

        assert_eq!(new_dir.files().borrow_mut().get(0).unwrap().as_ref(), &new_file);
    }
}