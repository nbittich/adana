use std::{
    fmt::Display,
    fs::{File, remove_file},
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use log::{debug, error};

#[derive(Debug, Clone)]
pub struct FileLock {
    _lock_p: PathBuf,
    inner_p: PathBuf,
}

fn pid_exists(pid: u32) -> bool {
    Path::new(&format!("/proc/{pid}")).exists()
}

#[derive(Debug)]
pub enum FileLockError {
    PidExist(u32),
    PidFileDoesntExist,
    Unknown(String),
}

impl Display for FileLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileLockError::PidExist(pid) => {
                write!(f, "Could not acquire lock (pid exists: {pid})")
            }
            FileLockError::PidFileDoesntExist => write!(
                f,
                "Lock exist but pid file doesn't! this is probably a bug."
            ),
            FileLockError::Unknown(e) => write!(f, "{e}"),
        }
    }
}

pub fn read_file(p: &PathBuf) -> anyhow::Result<BufReader<File>> {
    let _inner = File::options().read(true).open(p)?;
    let reader = BufReader::new(_inner);
    Ok(reader)
}

impl FileLock {
    pub fn get_path(&self) -> &PathBuf {
        &self.inner_p
    }
    pub fn open<P: AsRef<Path>>(path: P) -> Result<FileLock, FileLockError> {
        let _lock_p = path.as_ref().with_extension("lock");
        let inner_p = path.as_ref().to_path_buf();
        if Path::exists(&_lock_p) {
            let pid = Self::read_pid(&path);

            match pid {
                Ok(pid) => {
                    if pid_exists(pid) {
                        error!("{pid} exist!");
                        return Err(FileLockError::PidExist(pid));
                    }
                }
                _ => {
                    return Err(FileLockError::PidFileDoesntExist);
                }
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
        let _ = File::options()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| FileLockError::Unknown(e.to_string()))?;
        let _ = File::create(&_lock_p)
            .map_err(|e| FileLockError::Unknown(e.to_string()))?;
        Self::write_pid(&path)
            .map_err(|e| FileLockError::Unknown(e.to_string()))?;

        std::fs::copy(&path, &_lock_p)
            .map_err(|e| FileLockError::Unknown(e.to_string()))?;

        Ok(FileLock { _lock_p, inner_p })
    }

    pub fn read(&self) -> anyhow::Result<BufReader<File>> {
        read_file(&self.inner_p)
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

    pub fn flush(&self) -> anyhow::Result<()> {
        debug!("flush file");
        let swp = &self.inner_p.with_extension("swp");
        let _ = File::create(swp)?;
        let _ = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.inner_p)
            .unwrap();

        std::fs::rename(&self.inner_p, swp)?;

        std::fs::copy(&self._lock_p, &self.inner_p)
            .map_err(|e| anyhow::format_err!("{e}"))?;

        remove_file(swp)?;
        Ok(())
    }

    fn cleanup_and_flush(&mut self) -> anyhow::Result<()> {
        debug!("remove lock for {}", self._lock_p.as_path().to_string_lossy());

        let pid = &self.inner_p.with_extension("pid");

        self.flush()?;

        remove_file(&self._lock_p)?;
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

        if let Err(e) = open_file_twice {
            assert!(
                e.to_string()
                    .starts_with("Could not acquire lock (pid exists: ")
            );
        }
    }
}
