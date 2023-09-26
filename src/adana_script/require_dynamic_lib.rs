const STD_DOWNLOAD_URI: &str =
    "https://github.com/nbittich/adana-std/releases/download/0.0.1/adana-std.tar.gz";

use std::{
    path::Path,
    process::{Command, Stdio},
};

use adana_script_core::primitive::NativeLibrary;
use anyhow::{anyhow, Context};

pub fn require_dynamic_lib(
    path: &str,
    shared_lib: impl AsRef<Path> + Copy,
) -> anyhow::Result<NativeLibrary> {
    try_from_path(path, shared_lib)
}

fn try_from_path(
    file_path: &str,
    shared_lib: impl AsRef<Path> + Copy,
) -> anyhow::Result<NativeLibrary> {
    let curr_path =
        std::env::current_dir().context("no current dir! wasn't expected")?;
    if cfg!(test) {
        dbg!(&curr_path);
    }

    let mut file_path = {
        if file_path.starts_with("@std") {
            let mut lib = file_path.replace("@std", "adana-std");
            lib.push_str(".so");
            let mut shared_lib_pb = shared_lib.as_ref().to_path_buf();
            shared_lib_pb.push(lib);

            if cfg!(test) {
                dbg!(&shared_lib_pb);
            }
            if !shared_lib_pb.exists() {
                let sl = shared_lib.as_ref().to_str().context("no path")?;
                // try to download it
                eprintln!(
                    r#"std lib doesn't exist: {shared_lib_pb:?}.

Try to install it like so:
    - wget -P /tmp {STD_DOWNLOAD_URI}
    - mkdir {sl}/adana-std && tar xvzf /tmp/adana-std.tar.gz -C {sl}/adana-std

                "#
                );
                return Err(anyhow::anyhow!("std lib doesn't exist"));
            }
            shared_lib_pb
        } else {
            let temp_path = Path::new(&file_path);

            if cfg!(test) {
                dbg!(&temp_path);
            }

            let mut parent = temp_path
                .parent()
                .filter(|p| p.is_dir())
                .map(|p| p.to_path_buf())
                .or_else(|| Some(shared_lib.as_ref().to_path_buf()))
                .and_then(|p| p.canonicalize().ok())
                .context("parent or shared lib doesn't exist")?;
            if cfg!(test) {
                dbg!(&parent);
            }
            parent.push(temp_path.file_name().context("file name not found")?);
            parent
        }
    };
    if file_path.is_dir() && file_path.exists() {
        std::env::set_current_dir(&file_path)
            .map_err(|e| anyhow!("could not change dir: {e}"))?;
        println!("building lib {file_path:?}...");
        let mut handle = Command::new("cargo")
            .args(["build", "--release"])
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()?;

        let status_code = handle.wait()?;

        std::env::set_current_dir(curr_path)
            .map_err(|e| anyhow!("could not change dir: {e}"))?;

        if !status_code.success() {
            return Err(anyhow!("could not build library"));
        }
        file_path.push("target/release");
        if cfg!(test) {
            dbg!(&file_path);
        }
        for f in std::fs::read_dir(&file_path)? {
            let f = f?;
            let p = f.path();
            if let Some("so") = p.extension().and_then(|p| p.to_str()) {
                file_path.push(p);
                break;
            }
        }
    }

    if file_path.extension().and_then(|e| e.to_str()) != Some("so") {
        return Err(anyhow!("not a shared object!"));
    }
    println!("loading {file_path:?}");
    unsafe { NativeLibrary::new(file_path.as_path()) }
}
#[cfg(test)]
mod test {}
