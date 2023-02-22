use crate::{modules::*, terminal::Color};

#[cfg(feature = "time")]
use crate::modules::TimeScheme;

#[derive(Copy, Clone)]
pub struct SimpleTheme;

const WHITE: Color = Color(231);
const BLACK: Color = Color(16);

impl CmdScheme for SimpleTheme {
	const CMD_BG: Color = Color(237);
	const CMD_FG: Color = WHITE;
}

impl CwdScheme for SimpleTheme {
	const CWD_FG: Color = Color(255);
	const PATH_FG: Color = WHITE;
	const PATH_BG: Color = Color(237);
	const HOME_FG: Color = WHITE;
	const HOME_BG: Color = Color(75);
	const SEPARATOR_FG: Color = Color(244);
}

impl ExitCodeScheme for SimpleTheme {
	const EXIT_CODE_BG: Color = Color(1);
	const EXIT_CODE_FG: Color = WHITE;
}

impl UserScheme for SimpleTheme {
	const USERNAME_ROOT_BG: Color = Color(9);
	const USERNAME_BG: Color = Color(8);
	const USERNAME_FG: Color = Color(255);
}

impl FishScheme for SimpleTheme {
	const FISH_BG: Color = Color(220);
	const FISH_FG: Color = BLACK;
}

impl ExecTimeScheme for SimpleTheme {
	const EXEC_TIME_OK_BG: Color = Color(22);
	const EXEC_TIME_OK_FG: Color = WHITE;
	const EXEC_TIME_ERR_BG: Color = Color(52);
	const EXEC_TIME_ERR_FG: Color = WHITE;
}

impl NewLineScheme for SimpleTheme {}

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
	const TIME_BG: Color = BLACK;
	const TIME_FG: Color = Color(255);
}

impl GitScheme for SimpleTheme {
	const GIT_AHEAD_BEHIND_BG: Color = Color(240);
	const GIT_AHEAD_BEHIND_FG: Color = WHITE;
	const GIT_CONFLICTED_BG: Color = Color(160);
	const GIT_CONFLICTED_FG: Color = WHITE;
	const GIT_NOT_STAGED_BG: Color = Color(63);
	const GIT_NOT_STAGED_FG: Color = WHITE;
	const GIT_REPO_CLEAN_BG: Color = Color(190);
	const GIT_REPO_CLEAN_FG: Color = BLACK;
	const GIT_REPO_DIRTY_BG: Color = Color(161);
	const GIT_REPO_DIRTY_FG: Color = Color(255);
	const GIT_REPO_ERROR_BG: Color = Color(9);
	const GIT_REPO_ERROR_FG: Color = WHITE;
	const GIT_REPO_NO_UPSTREAM_BG: Color = Color(240);
	const GIT_REPO_NO_UPSTREAM_FG: Color = BLACK;
	const GIT_STAGED_BG: Color = Color(142);
	const GIT_STAGED_FG: Color = BLACK;
	const GIT_UNTRACKED_BG: Color = Color(63);
	const GIT_UNTRACKED_FG: Color = WHITE;
}

impl PyVenvScheme for SimpleTheme {
	const PYVENV_FG: Color = Color(12);
	const PYVENV_BG: Color = Color(0);
}
