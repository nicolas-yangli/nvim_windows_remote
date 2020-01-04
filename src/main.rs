use std::path::Path;

use nvim_windows_remote::*;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let filepath = Path::new(&filename);
    edit_with_existing_nvim(filepath).unwrap();
}
