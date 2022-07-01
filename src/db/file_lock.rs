use std::{
    fs::{remove_file, File},
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::{prelude::debug, utils::pid_exists};

#[derive(Debug, Clone)]
pub struct FileLock {
    _lock_p: PathBuf,
    inner_p: PathBuf,
}

impl FileLock {
    pub fn open<P: AsRef<Path>>(path: P) -> anyhow::Result<FileLock> {
        let _lock_p = path.as_ref().with_extension("lock");
        let inner_p = path.as_ref().to_path_buf();
        if Path::exists(&_lock_p) {
            let pid = Self::read_pid(&path);

            if let Ok(pid) = pid {
                if pid_exists(pid) {
                    debug!("{pid} exist!");
                    return Err(anyhow::Error::msg("Could not acquire lock"));
                }
            } else {
                return Err(anyhow::Error::msg(
                    "Lock exist but pid file doesn't! this is probably a bug.",
                ));
            }

            // otherwise, we create a file lock to force cleanup
            let _ = {
                let _ = FileLock {
                    _lock_p: _lock_p.clone(),
                    inner_p: inner_p.clone(),
                };
                Some(())
            };
        }
        // create files if not exist
        let _ = File::options().create(true).append(true).open(&path)?;
        let _ = File::create(&_lock_p)?;
        Self::write_pid(&path)?;

        std::fs::copy(&path, &_lock_p)?;

        Ok(FileLock { _lock_p, inner_p })
    }

    pub fn read(&self) -> anyhow::Result<BufReader<File>> {
        let _inner = File::options().read(true).open(&self.inner_p)?;
        let reader = BufReader::new(_inner);
        Ok(reader)
    }

    pub fn write(&self, buf: &[u8]) -> anyhow::Result<()> {
        let _lock = File::create(&self._lock_p)?;
        let mut writer = BufWriter::new(_lock);
        writer.write_all(buf)?;
        writer.flush()?;
        Ok(())
    }

    fn write_pid<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
        let pid_p = path.as_ref().with_extension("pid");
        let pid_id = std::process::id();
        std::fs::write(pid_p, pid_id.to_string().as_bytes())?;
        Ok(())
    }

    fn read_pid<P: AsRef<Path>>(path: P) -> anyhow::Result<u32> {
        let pid_p = path.as_ref().with_extension("pid");
        let pid = std::fs::read_to_string(pid_p)?;
        Ok(str::parse::<u32>(&pid)?)
    }

    fn cleanup_and_flush(&mut self) -> anyhow::Result<()> {
        debug!("remove lock for {}", self._lock_p.as_path().to_string_lossy());
        let swp = &self.inner_p.with_extension("swp");
        let pid = &self.inner_p.with_extension("pid");
        let _ = File::create(swp)?;
        let _ = File::options()
            .write(true)
            .create(true)
            .open(&self.inner_p)
            .unwrap();

        std::fs::rename(&self.inner_p, swp)?;

        std::fs::rename(&self._lock_p, &self.inner_p)?;

        remove_file(swp)?;
        remove_file(pid)?;

        Ok(())
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        self.cleanup_and_flush().unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::io::BufRead;

    use super::FileLock;

    #[test]
    fn test_lock() {
        let path = "/tmp/db.json";
        let file = FileLock::open(path).unwrap();
        let text = "\ni wanna wanna way";

        file.write(text.as_bytes()).unwrap();

        let mut reader = file.read().unwrap();
        let mut line = String::new();
        let len = reader.read_line(&mut line).unwrap();
        println!("First line is {len} bytes long");

        let open_file_twice = FileLock::open(path);

        match open_file_twice {
            Err(e) => assert_eq!(e.to_string(), "Could not acquire lock"),
            Ok(_) => (),
        }
    }
}
