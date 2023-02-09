use crate::{modules::*, terminal::Color};

#[cfg(feature = "time")]
use crate::modules::TimeScheme;

#[derive(Copy, Clone)]
pub struct SimpleTheme;

impl CmdScheme for SimpleTheme {
	const CMD_PASSED_BG: Color = Color(237);
	const CMD_PASSED_FG: Color = Color(15);
	const CMD_FAILED_BG: Color = Color(9);
	const CMD_FAILED_FG: Color = Color(15);
}

impl CwdScheme for SimpleTheme {
	const CWD_FG: Color = Color(255);
	const PATH_FG: Color = Color(15);
	const PATH_BG: Color = Color(237);
	const HOME_FG: Color = Color(15);
	const HOME_BG: Color = Color(75);
	const SEPARATOR_FG: Color = Color(244);
}

impl ExitCodeScheme for SimpleTheme {
	const EXIT_CODE_BG: Color = Color(1);
	const EXIT_CODE_FG: Color = Color(15);
}

impl UserScheme for SimpleTheme {
	const USERNAME_ROOT_BG: Color = Color(9);
	const USERNAME_BG: Color = Color(8);
	const USERNAME_FG: Color = Color(255);
}

impl FishScheme for SimpleTheme {
	const FISH_BG: Color = Color(220);
	const FISH_FG: Color = Color(0);
}

impl HostScheme for SimpleTheme {
	const HOSTNAME_FG: Color = Color(255);
	const HOSTNAME_BG: Color = Color(8);
}

impl ReadOnlyScheme for SimpleTheme {
	const READONLY_FG: Color = Color(255);
	const READONLY_BG: Color = Color(1);
}

#[cfg(feature = "time")]
impl TimeScheme for SimpleTheme {
	const TIME_BG: Color = Color(0);
	const TIME_FG: Color = Color(255);
}

impl GitScheme for SimpleTheme {
	const GIT_AHEAD_BG: Color = Color(240);
	const GIT_AHEAD_FG: Color = Color(15);
	const GIT_BEHIND_BG: Color = Color(240);
	const GIT_BEHIND_FG: Color = Color(15);
	const GIT_STAGED_BG: Color = Color(12);
	const GIT_STAGED_FG: Color = Color(15);
	const GIT_NOTSTAGED_BG: Color = Color(5);
	const GIT_NOTSTAGED_FG: Color = Color(15);
	const GIT_UNTRACKED_BG: Color = Color(11);
	const GIT_UNTRACKED_FG: Color = Color(15);
	const GIT_CONFLICTED_BG: Color = Color(9);
	const GIT_CONFLICTED_FG: Color = Color(15);
	const GIT_REPO_CLEAN_BG: Color = Color(10);
	const GIT_REPO_CLEAN_FG: Color = Color(0);
	const GIT_REPO_DIRTY_BG: Color = Color(9);
	const GIT_REPO_DIRTY_FG: Color = Color(15);
	const GIT_REPO_ERROR_BG: Color = Color(9);
	const GIT_REPO_ERROR_FG: Color = Color(15);
}

impl PyVenvScheme for SimpleTheme {
	const PYVENV_FG: Color = Color(12);
	const PYVENV_BG: Color = Color(0);
}
