use fs_lib::cli::*;

fn main() {
    println!("Hello World");
    let new_session = Session::new();
    new_session.run().unwrap();
    
}
