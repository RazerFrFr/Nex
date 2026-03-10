mod backend;
mod game;
pub mod launcher;

pub const CANNOT_CONNECT_ERRORS: &[&str] = &[
    "port 3551 failed: Connection refused",
    "Unable to login to Fortnite servers",
    "HTTP 400 response from ",
    "Network failure when attempting to check platform restrictions",
    "UOnlineAccountCommon::ForceLogout",
];

pub const LOGGED_IN_LINES: &[&str] = &["[UOnlineAccountCommon::ContinueLoggingIn]", "(Completed)"];
