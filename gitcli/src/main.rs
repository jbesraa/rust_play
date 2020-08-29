use chrono::Duration;
use chrono::NaiveDateTime;
use crossterm::ErrorKind;
use git2::Oid;
use git2::{BranchType, Repository};
use std::convert::TryFrom;
use std::io;
use std::io::Bytes;
use std::io::Stdin;
use std::io::Stdout;
use std::io::{Read, Write};
use std::string::FromUtf8Error;

fn main() {
    let result = || -> Result<_> {
        let repo = Repository::open_from_env()?;

        crossterm::terminal::enable_raw_mode()?;

        let mut stdout = io::stdout();
        let mut stdin = io::stdin().bytes();
        let mut branches = get_branches(&repo)?;
        if branches.is_empty() {
            write!(stdout, "Found no branches (master's ignored)")?;
            write!(stdout, "\r\n")?;
        } else {
            for branch in &mut branches {
                if branch.is_head {
                    write!(stdout, "ignored {} because its head", branch.name)?;
                } else {
                    match get_branch_action_from_user(&mut stdout, &mut stdin, &branch)? {
                        BranchAction::Keep => {
                            write!(stdout, "")?;
                        }
                        BranchAction::Delete => {
                            branch.delete()?;

                            write!(
                                stdout,
                                "deleted {}, to undo run `git branch {} {}`\r\n",
                                branch.name, branch.name, branch.id
                            )?;
                        }
                        BranchAction::Quit => return Ok(()),
                    }
                }
            }
        }
        Ok(())
    }();
    crossterm::terminal::disable_raw_mode().ok();
    match result {
        Ok(()) => {}
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1)
        }
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn get_branch_action_from_user(
    stdout: &mut Stdout,
    stdin: &mut Bytes<Stdin>,
    branch: &Branch,
) -> Result<BranchAction> {
    write!(
        stdout,
        "'{}' ({}) last commit at {} (k/d/q/?) >",
        branch.name,
        &branch.id.to_string()[0..10],
        branch.time
    )?;

    stdout.flush()?;

    let byte = match stdin.next() {
        Some(byte) => byte?,
        None => return get_branch_action_from_user(stdout, stdin, branch),
    };

    let c = char::from(byte);
    write!(stdout, "{}\r\n", c)?;
    if c == '?' {
        write!(stdout, "\r\n")?;
        write!(stdout, "Here are what the commands mean \r\n")?;
        write!(stdout, "k - keep the branch\r\n")?;
        write!(stdout, "d - delete the branch \r\n")?;
        write!(stdout, "q - quite \r\n")?;
        write!(stdout, "? - show this help text \r\n")?;
        stdout.flush()?;
        get_branch_action_from_user(stdout, stdin, branch)
    } else {
        BranchAction::try_from(c)
    }
}

fn get_branches(repo: &Repository) -> Result<Vec<Branch>> {
    let mut branches = repo
        .branches(Some(BranchType::Local))?
        .map(|branch| {
            let (branch, _) = branch?;

            let name = String::from_utf8(branch.name_bytes()?.to_vec())?;

            let commit = branch.get().peel_to_commit()?;
            let time = commit.time();
            let offset = Duration::minutes(i64::from(time.offset_minutes()));
            let time = NaiveDateTime::from_timestamp(time.seconds(), 0) + offset;

            Ok(Branch {
                id: commit.id(),
                time,
                is_head: branch.is_head(),
                name,
                branch,
            })
        })
        .filter(|branch| match branch {
            Ok(branch) => branch.name != "master",
            Err(_) => true,
        })
        .collect::<Result<Vec<_>>>()?;
    branches.sort_by_key(|branch| branch.time);
    Ok(branches)
}

struct Branch<'repo> {
    id: Oid,
    time: NaiveDateTime,
    name: String,
    is_head: bool,
    branch: git2::Branch<'repo>,
}

impl<'repo> Branch<'repo> {
    fn delete(&mut self) -> Result<()> {
        self.branch.delete().map_err(From::from)
    }
}
enum BranchAction {
    Keep,
    Delete,
    Quit,
}

impl TryFrom<char> for BranchAction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'k' => Ok(BranchAction::Keep),
            'd' => Ok(BranchAction::Delete),
            'q' => Ok(BranchAction::Quit),
            _ => Err(Error::InvalidInput(value)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    CrosstermError(#[from] ErrorKind),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    GitError(#[from] git2::Error),

    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("Invalid input. Dont know what '{0} means")]
    InvalidInput(char),
}
