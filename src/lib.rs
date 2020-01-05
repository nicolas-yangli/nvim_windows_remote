use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::io::Write;

#[macro_use]
extern crate clap;

mod nvim_rpc;

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum EditCommand {
        edit,
        split,
        vsplit,
    }
}

impl EditCommand {
    fn to_str(&self) -> &'static str {
        match self {
            EditCommand::edit => "edit",
            EditCommand::split => "split",
            EditCommand::vsplit => "vsplit",
        }
    }
}

pub fn edit_with_existing_nvim(edit_command: EditCommand, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.to_str().unwrap();
    let nvim_named_pipe = find_existing_nvim_named_pipe().unwrap();
    let mut pipe = OpenOptions::new().read(true).write(true).create(false).open(nvim_named_pipe).unwrap();
    let mut session = nvim_rpc::Session::new();
    let mut notify = session.notify(&mut pipe);
    notify.method("nvim_command")?;
    notify.param_count(1)?;
    notify.param(&*format!("exe '{}' fnameescape('{}')", edit_command.to_str(), path.replace("'", "\\'")),)?;
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
