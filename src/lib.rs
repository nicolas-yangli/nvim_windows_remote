use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::io::{stdout, Write};

mod nvim_rpc;

pub fn edit_with_existing_nvim(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.to_str().unwrap();
    let nvim_named_pipe = find_existing_nvim_named_pipe().unwrap();
    let mut pipe = OpenOptions::new().read(true).write(true).create(false).open(nvim_named_pipe).unwrap();
    let mut session = nvim_rpc::Session::new();
    let mut notify = session.notify(&mut pipe);
    notify.method("nvim_command")?;
    notify.param_count(1)?;
    notify.param(&*format!("exe 'edit' fnameescape('{}')", path.replace("'", "\\'")))?;
    pipe.flush()?;
    Ok(())
}

pub fn find_existing_nvim_named_pipe() -> Option<PathBuf> {
    for entry in Path::new("\\\\.\\pipe\\").read_dir().unwrap() {
        let entry = entry.unwrap();
        if entry.file_name().into_string().unwrap().starts_with("nvim-") {
            return Some(entry.path());
        }
    }
    None
}
