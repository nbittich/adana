use crate::prelude::*;

pub fn exec_command(command: &str) -> Res<()> {
    let (rest, program) = preceded(multispace0, take_while(|s| s != ' '))(command)?;

    let (_, args) = preceded(
        multispace0,
        separated_list0(
            tag(" "),
            verify(take_while(|s| s != ' '), |s: &str| !s.is_empty()),
        ),
    )(rest)?;

    let handle = Command::new(program)
        .args(&args[..])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match handle.and_then(|mut h| h.wait()) {
        Ok(status) => println!("{status}"),
        Err(e) => eprintln!("{program} failed to start with args {args:?}. err: {e}"),
    }

    Ok((command, ()))
}

#[cfg(test)]
mod test {
    use super::exec_command;

    #[test]
    fn test_exec_command() {
        exec_command("echo 'hello world'").unwrap();
        println!("bye")
    }
}
