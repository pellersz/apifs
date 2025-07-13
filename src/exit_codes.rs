pub enum ExitCode {
    Finished(1),
    WrongArguments, 
    ScriptIssue,
    AlreadyRunning,
    ServerRunError,
}
