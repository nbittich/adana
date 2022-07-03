use anyhow::Context;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{self, MatchingBracketValidator, Validator};
use rustyline::{
    Cmd, CompletionType, Config, Context as Ctx, EditMode, Editor, KeyEvent,
};
use rustyline_derive::Helper;
use std::borrow::Cow::{self, Borrowed, Owned};
use std::path::Path;

fn get_default_history_path() -> Option<Box<Path>> {
    let mut home_dir = dirs::home_dir()?;
    home_dir.push(".karsher.history.txt");
    Some(home_dir.into_boxed_path())
}

pub fn save_history(
    rl: &mut Editor<CustomHelper>,
    history_path: Option<impl AsRef<Path>>,
) -> anyhow::Result<()> {
    let history_path = history_path
        .map(|p| p.as_ref().into())
        .or_else(get_default_history_path)
        .context("history path not found")?;
    rl.save_history(history_path.as_ref())
        .map_err(|e| anyhow::Error::msg(format!("{e}")))
}

pub fn read_line(
    rl: &mut Editor<CustomHelper>,
    curr_cache: &str,
) -> Result<String, rustyline::error::ReadlineError> {
    let p = format!("[{curr_cache}] >> ");
    rl.helper_mut().expect("No helper").colored_prompt =
        format!("\x1b[1;32m{}\x1b[0m", p);
    rl.readline(&p)
}

pub fn build_editor(
    history_path: Option<impl AsRef<Path>>,
) -> Editor<CustomHelper> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Vi)
        .build();
    let h = CustomHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::with_config(config);
    rl.set_helper(Some(h));
    rl.bind_sequence(KeyEvent::ctrl('d'), Cmd::Interrupt);
    rl.bind_sequence(KeyEvent::ctrl('c'), Cmd::Undo(1));
    rl.bind_sequence(KeyEvent::ctrl('l'), Cmd::ClearScreen);
    rl.bind_sequence(KeyEvent::alt('n'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('p'), Cmd::HistorySearchBackward);
    if rl
        .load_history(
            history_path
                .map(|p| p.as_ref().into())
                .or_else(get_default_history_path)
                .context("history path missing")
                .unwrap()
                .as_ref(),
        )
        .is_err()
    {
        println!("No previous history.");
    }
    rl
}

#[derive(Helper)]
pub struct CustomHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    colored_prompt: String,
}
impl Highlighter for CustomHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for CustomHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}

impl Completer for CustomHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Ctx<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for CustomHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Ctx<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}
