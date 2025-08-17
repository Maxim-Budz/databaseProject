// CLI entry point
//

mod file_manager;
use crate::file_manager::file_manager::build_file_manager;

fn main() {
    let file_manager = build_file_manager(5, String::from("./") );
}
