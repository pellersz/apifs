pub enum ExitCode {
    #[allow(dead_code)]
    Finished,
    WrongArguments, 
    FileError,
    AlreadyRunning,
    ServerRunError,
    PathError,
}
