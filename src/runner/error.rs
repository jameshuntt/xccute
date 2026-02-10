use std::io;

#[derive(Debug)]
pub enum CommandError {
    Io(io::Error),
    ExitFailure(Option<i32>),
    ExecutionFailed(String),

}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Io(e) => write!(f, "IO error: {}", e),
            CommandError::ExitFailure(code) => {
                write!(f, "Command exited with code: {:?}", code)
            }
            CommandError::ExecutionFailed(cmd) => write!(f, "Command failed: {}", cmd),

        }
    }
}

impl std::error::Error for CommandError {}




// use std::fmt;
// use std::io;
// 
// #[derive(Debug)]
// pub enum CommandError {
//     Io(io::Error),
//     ExecutionFailed(String),
// }
// 
// impl fmt::Display for CommandError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             CommandError::Io(err) => write!(f, "IO error: {}", err),
//             CommandError::ExecutionFailed(cmd) => write!(f, "Command failed: {}", cmd),
//         }
//     }
// }
// 
// impl std::error::Error for CommandError {}
// 
impl From<io::Error> for CommandError {
    fn from(err: io::Error) -> Self {
        CommandError::Io(err)
    }
}
