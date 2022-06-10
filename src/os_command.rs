use crate::prelude::*;

pub fn exec_command<'a>(command: &'a str, extra_args: &'a Option<&'a str>) -> Res<'a, ()> {
    let (remaining, program) = preceded(multispace0, take_while(|s| s != ' '))(command)?;

    let extract_args = |s| {
        preceded(
            multispace0,
            separated_list0(
                tag(" "),
                alt((
                    delimited(tag("\""), take_while(|s: char| s != '"'), tag("\"")),
                    verify(take_while(|s: char| !s.is_whitespace()), |s: &str| {
                        !s.is_empty()
                    }),
                )),
            ),
        )(s)
    };
    let (_, mut args) = extract_args(remaining)?;

    if let Some(extra_args) = extra_args {
        let (_, mut extra_args) = extract_args(extra_args)?;
        args.append(&mut extra_args);
    }

    let handle = Command::new(program)
        .args(&args[..])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match handle.and_then(|mut h| h.wait()) {
        Ok(status) => {
            if cfg!(debug_assertions) {
                println!("{status}");
            }
        }
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
