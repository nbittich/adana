use std::ffi::OsString;

use crate::prelude::*;
fn extract_args(s: &str) -> Res<Vec<OsString>> {
    preceded(
        multispace0,
        separated_list0(
            multispace1,
            map(
                alt((
                    delimited(
                        tag("\""),
                        take_while(|s: char| s != '"'),
                        tag("\""),
                    ),
                    verify(
                        take_while(|s: char| !s.is_whitespace()),
                        |s: &str| !s.is_empty(),
                    ),
                )),
                |s: &str| {
                    if s.starts_with('$') {
                        let key = s.replace('$', "");
                        let env = std::env::var_os(key);
                        if let Some(env) = env {
                            env
                        } else {
                            OsString::from(s)
                        }
                    } else {
                        s.into()
                    }
                },
            ),
        ),
    )(s)
}

fn extract_envs(s: &str) -> Res<Vec<(&str, &str)>> {
    preceded(
        multispace0,
        separated_list0(
            space1,
            separated_pair(take_until1("="), tag("="), take_until(" ")),
        ),
    )(s)
}

fn extract_program(s: &str) -> Res<&str> {
    preceded(multispace0, take_while(|s| s != ' '))(s)
}

pub fn exec_command<'a>(
    command: &'a str,
    extra_args: &'a Option<&'a str>,
) -> Res<'a, ()> {
    let (remaining, envs) = extract_envs(command)?;
    let (remaining, program) = extract_program(remaining)?;

    let (_, mut args) = extract_args(remaining)?;

    if let Some(extra_args) = extra_args {
        let (_, mut extra_args) = extract_args(extra_args)?;
        args.append(&mut extra_args);
    }

    let handle = Command::new(program)
        .envs(envs)
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    match handle.and_then(|mut h| h.wait()) {
        Ok(status) => {
            debug!("{status}");
        }
        Err(e) => {
            eprintln!("{program} failed to start with args {args:?}. err: {e}")
        }
    }

    Ok((command, ()))
}

#[cfg(test)]
mod test {
    use super::exec_command;

    #[test]
    fn test_exec_command() {
        exec_command("echo 'hello world'", &None).unwrap();
        println!("bye")
    }
}
