use fs_lib::file;

fn main() {
    let _new_file = file::File::new(String::from("file"), String::from("txt"), 1);
    println!("Hello World");
}
