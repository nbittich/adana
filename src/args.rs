use anyhow::Context;

pub enum Argument {
    InMemory,
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
                arguments.push(Argument::SharedLibPath(path));
            }

            "--sharedlibpath" | "-slp" => {
                anyhow::ensure!(
                    !arguments
                        .iter()
                        .any(|a| matches!(a, Argument::SharedLibPath(_))),
                    "shared lib path should be specified only once!"
                );
                let path = args.next().context("history path missing!!")?;
                arguments.push(Argument::HistoryPath(path));
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
