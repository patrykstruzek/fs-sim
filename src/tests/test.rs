#[cfg(test)]
mod tests {
    use fs_lib::dir::*;
    use fs_lib::fl::*;
    use std::cell::RefMut;
    use std::rc::Rc;

    #[test]
    fn dir_should_store_file() {
        let mut new_dir = Directory::new("dir");
        let new_file = File::new("file", "txt", 0);

        new_dir.store(new_file.clone());

        let file_ref: Rc<File> = Rc::new(new_file);
        let stored_files: RefMut<Vec<Rc<File>>> = new_dir.files().borrow_mut();

        assert_eq!(stored_files.get(0), Some(&file_ref));
    }
}
