pub enum ExitCode {
    #[allow(dead_code)]
    Finished,
    WrongArguments, 
    ScriptIssue,
    AlreadyRunning,
    ServerRunError,
    PathError
}
