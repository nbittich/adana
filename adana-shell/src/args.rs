use anyhow::Context;

pub enum Argument {
    InMemory,
    Execute(String),
    Daemon,
    ScriptPath(String),
    DbPath(String),
    NoFallbackInMemory,
    HistoryPath(String),
    SharedLibPath(String),
    DefaultCache(String),
}

pub fn parse_args(
    mut args: impl Iterator<Item = String>,
) -> anyhow::Result<Vec<Argument>> {
    let mut arguments = vec![];
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--scriptpath" | "-sp" => {
                anyhow::ensure!(
                    !arguments
                        .iter()
                        .any(|a| matches!(a, Argument::ScriptPath(_))),
                    "script path should be specified only once!"
                );
                let path = args.next().context("script path missing!!")?;
                arguments.push(Argument::ScriptPath(path));
            }
            "--execute" | "-e" => {
                anyhow::ensure!(
                    !arguments
                        .iter()
                        .any(|a| matches!(a, Argument::Execute(_))),
                    "execute should be specified only once!"
                );
                let mut rest = String::new();

                for a in &mut args {
                    rest.push(' ');
                    println!("{a}");
                    rest.push_str(&a);
                }
                arguments.push(Argument::Execute(rest));
                break;
            }
            "--inmemory" | "-im" => {
                anyhow::ensure!(
                    !arguments.iter().any(|a| matches!(a, Argument::InMemory)),
                    "in memory should be specified only once!"
                );
                anyhow::ensure!(
                    !arguments.iter().any(|a| matches!(a, Argument::DbPath(_))),
                    "cannot have db path & in memory at the same time!"
                );
                arguments.push(Argument::InMemory);
            }
            "--daemon" | "-d" => {
                anyhow::ensure!(
                    !arguments.iter().any(|a| matches!(a, Argument::InMemory)),
                    "daemon should be specified only once!"
                );
                anyhow::ensure!(
                    arguments
                        .iter()
                        .any(|a| matches!(a, Argument::ScriptPath(_))),
                    "script path must be specified first when having the daemon feature on! "
                );
                arguments.push(Argument::Daemon);
            }
            "--no-fallback" | "-nofb" => {
                anyhow::ensure!(
                    !arguments
                        .iter()
                        .any(|a| matches!(a, Argument::NoFallbackInMemory)),
                    "no fallback should be specified only once!"
                );
                arguments.push(Argument::NoFallbackInMemory);
            }
            "--dbpath" | "-db" => {
                anyhow::ensure!(
                    !arguments.iter().any(|a| matches!(a, Argument::InMemory)),
                    "cannot mix in memory & db path!"
                );
                anyhow::ensure!(
                    !arguments.iter().any(|a| matches!(a, Argument::DbPath(_))),
                    "db path should be specified only once!"
                );
                let path = args.next().context("db path missing!!")?;
                arguments.push(Argument::DbPath(path));
            }
            "--historypath" | "-hp" => {
                anyhow::ensure!(
                    !arguments
                        .iter()
                        .any(|a| matches!(a, Argument::HistoryPath(_))),
                    "history path should be specified only once!"
                );
                let path = args.next().context("history path missing!!")?;
                arguments.push(Argument::HistoryPath(path));
            }

            "--sharedlibpath" | "-slp" => {
                anyhow::ensure!(
                    !arguments
                        .iter()
                        .any(|a| matches!(a, Argument::SharedLibPath(_))),
                    "shared lib path should be specified only once!"
                );
                let path = args.next().context("shared lib path missing!!")?;
                arguments.push(Argument::SharedLibPath(path));
            }
            "--cache" | "-c" => {
                let default_cache =
                    args.next().context("default cache missing!!")?;
                arguments.push(Argument::DefaultCache(default_cache));
            }

            _ => (), // ignore unknown argument
        }
    }
    Ok(arguments)
}
