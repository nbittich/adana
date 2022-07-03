use anyhow::Context;

pub enum Argument {
    InMemory,
    DbPath(String),
    FallbackInMemory,
    HistoryPath(String),
}

pub fn parse_args(
    mut args: impl Iterator<Item = String>,
) -> anyhow::Result<Vec<Argument>> {
    let mut arguments = vec![];
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--inmemory" => {
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
            "--fallback" => {
                anyhow::ensure!(
                    !arguments
                        .iter()
                        .any(|a| matches!(a, Argument::FallbackInMemory)),
                    "fallback should be specified only once!"
                );
                arguments.push(Argument::FallbackInMemory);
            }
            "--dbpath" => {
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
            "--historypath" => {
                anyhow::ensure!(
                    !arguments
                        .iter()
                        .any(|a| matches!(a, Argument::HistoryPath(_))),
                    "history path should be specified only once!"
                );
                let path = args.next().context("history path missing!!")?;
                arguments.push(Argument::HistoryPath(path));
            }

            _ => (), // ignore unknown argument
        }
    }
    Ok(arguments)
}
