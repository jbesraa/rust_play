use chrono::Duration;
use chrono::NaiveDateTime;
use crossterm::ErrorKind;
use git2::Oid;
use git2::{BranchType, Repository};
use std::io;
use std::io::{Read, Write};
use std::string::FromUtf8Error;

fn main() -> Result<(), Error> {
    let repo = Repository::open_from_env()?;

    Ok(())
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn get_branches(repo: &Repository) -> Result<Vec<Branch>> {
    repo.branches(Some(BranchType::Local))?
        .map(|branch| {
            let (branch, _) = branch?;

            let name = String::from_utf8(branch.name_bytes()?.to_vec());

            let commit = branch.get().peel_to_commit()?;
            let time = commit.time();
            let offset = Duration::minutes(i64::from(time.offset_minutes()));
            let time = NaiveDateTime::from_timestamp(time.seconds(), 0) + offset;

            Ok(Branch {
                id: commit.id(),
                time,
                name,
            })
        })
        .collect()
}

#[derive(Debug)]
struct Branch {
    id: Oid,
    time: NaiveDateTime,
    name: String,
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
}

// impl From<ErrorKind> for Error {
//     fn from(error: ErrorKind) -> Error {
//         Error::CrosstermError(error)
//     }
// }

// impl From<io::Error> for Error {
//     fn from(error:io::Error ) -> Error {
//         Error::IoError(error)
//     }
// }
// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Error::CrosstermError(inner) => write!(f, "{}", inner),
//             Error::IoError(inner) => write!(f, "{}", inner),
//         }
//     }
// }

// impl std::error::Error for Error {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             Error::CrosstermError(inner) => Some(inner),
//             Error::IoError(inner) => Some(inner)
//         }
//     }
// }

// fn main() -> Result<(), Error> {
//     crossterm::terminal::enable_raw_mode()?;

//     let mut stdout = io::stdout();
//     let mut stdin = io::stdin().bytes();

//     loop {
//         write!(stdout, "Type Something > ")?;

//         stdout.flush()?;

//         let byte = match stdin.next() {
//             Some(byte) => byte?,
//             None => break,
//         };

//         let c = char::from(byte);

//         if c == 'q' {
//             break;
//         }
//         write!(stdout, "You typed '{}'\n\r", c)?;
//         stdout.flush()?;
//     }
//     crossterm::terminal::disable_raw_mode()?;

//     Ok(())
// }
