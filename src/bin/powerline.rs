extern crate powerline;

use std::env;

#[cfg(feature = "time")]
use powerline::modules::Time;
use powerline::{modules::*, theme::SimpleTheme};

const CMD_DISABLED_FLAG: &str = "-cmd";
const CMD_ENABLED_FLAG: &str = "cmd";
const CWD_DISABLED_FLAG: &str = "-cwd";
const CWD_ENABLED_FLAG: &str = "cwd";
const EXEC_TIME_DISABLED_FLAG: &str = "-exec_time";
const EXEC_TIME_ENABLED_FLAG: &str = "exec_time";
const EXIT_CODE_DISABLED_FLAG: &str = "-exit_code";
const EXIT_CODE_ENABLED_FLAG: &str = "exit_code";
const FISH_DISABLED_FLAG: &str = "-fish";
const FISH_ENABLED_FLAG: &str = "fish";
const GIT_DISABLED_FLAG: &str = "-git";
const GIT_ENABLED_FLAG: &str = "git";
const HOST_DISABLED_FLAG: &str = "-host";
const HOST_ENABLED_FLAG: &str = "host";
const NEW_LINE_CMD_DISABLED_FLAG: &str = "-nl_cmd";
const NEW_LINE_CMD_ENABLED_FLAG: &str = "nl_cmd";
const NEW_LINE_START_DISABLED_FLAG: &str = "-nl_start";
const NEW_LINE_START_ENABLED_FLAG: &str = "nl_start";
const PYVENV_DISABLED_FLAG: &str = "-pyvenv";
const PYVENV_ENABLED_FLAG: &str = "pyvenv";
const READONLY_DISABLED_FLAG: &str = "-readonly";
const READONLY_ENABLED_FLAG: &str = "readonly";
const USER_DISABLED_FLAG: &str = "-user";
const USER_ENABLED_FLAG: &str = "user";

#[cfg(feature = "time")]
const TIME_ENABLED_FLAG: &str = "time";
#[cfg(feature = "time")]
const TIME_DISABLED_FLAG: &str = "-time";

fn main() {
	let mut prompt = powerline::Powerline::new();

	let mut cmd_enabled = true;
	let mut cwd_enabled = true;
	let mut exec_time_enabled = true;
	let mut exit_code_enabled = true;
	let mut fish_enabled = true;
	let mut git_enabled = true;
	let mut host_enabled = false;
	let mut new_line_before_cmd_enabled = true;
	let mut new_line_start_enabled = true;
	let mut pyvenv_enabled = true;
	let mut readonly_enabled = true;
	let mut user_enabled = false;

	#[cfg(feature = "time")]
	let mut time_enabled = true;

	if cfg!(feature = "cli-options") {
		for arg in env::args() {
			match arg.as_str() {
				CMD_DISABLED_FLAG => cmd_enabled = false,
				CMD_ENABLED_FLAG => cmd_enabled = true,
				CWD_DISABLED_FLAG => cwd_enabled = false,
				CWD_ENABLED_FLAG => cwd_enabled = true,
				EXEC_TIME_DISABLED_FLAG => exec_time_enabled = false,
				EXEC_TIME_ENABLED_FLAG => exec_time_enabled = true,
				EXIT_CODE_DISABLED_FLAG => exit_code_enabled = false,
				EXIT_CODE_ENABLED_FLAG => exit_code_enabled = true,
				FISH_DISABLED_FLAG => fish_enabled = false,
				FISH_ENABLED_FLAG => fish_enabled = true,
				GIT_DISABLED_FLAG => git_enabled = false,
				GIT_ENABLED_FLAG => git_enabled = true,
				HOST_DISABLED_FLAG => host_enabled = false,
				HOST_ENABLED_FLAG => host_enabled = true,
				NEW_LINE_CMD_DISABLED_FLAG => new_line_before_cmd_enabled = false,
				NEW_LINE_CMD_ENABLED_FLAG => new_line_before_cmd_enabled = true,
				NEW_LINE_START_DISABLED_FLAG => new_line_start_enabled = false,
				NEW_LINE_START_ENABLED_FLAG => new_line_start_enabled = true,
				PYVENV_DISABLED_FLAG => pyvenv_enabled = false,
				PYVENV_ENABLED_FLAG => pyvenv_enabled = true,
				READONLY_DISABLED_FLAG => readonly_enabled = false,
				READONLY_ENABLED_FLAG => readonly_enabled = true,
				USER_DISABLED_FLAG => user_enabled = false,
				USER_ENABLED_FLAG => user_enabled = true,
				_ => {},
			}
			#[cfg(feature = "time")]
			{
				match arg.as_str() {
					TIME_ENABLED_FLAG => time_enabled = true,
					TIME_DISABLED_FLAG => time_enabled = false,
					_ => {},
				}
			}
		}
	}

	if new_line_start_enabled {
		measure_elapsed("new_line_start", || prompt.add_module(NewLine::<SimpleTheme>::new()));
	}
	if fish_enabled {
		measure_elapsed("fish", || prompt.add_module(Fish::<SimpleTheme>::new()));
	}
	if exec_time_enabled {
		measure_elapsed("exec_time", || prompt.add_module(ExecTime::<SimpleTheme>::new()));
	}
	if exit_code_enabled {
		measure_elapsed("exit_code", || prompt.add_module(ExitCode::<SimpleTheme>::new()));
	}
	#[cfg(feature = "time")]
	{
		if time_enabled {
			measure_elapsed("time", || prompt.add_module(Time::<SimpleTheme>::with_time_format("%H:%M:%S")));
		}
	}
	if pyvenv_enabled {
		measure_elapsed("pyvenv", || prompt.add_module(PyVenv::<SimpleTheme>::new()));
	}
	if user_enabled {
		measure_elapsed("user", || prompt.add_module(User::<SimpleTheme>::new()));
	}
	if host_enabled {
		measure_elapsed("host", || prompt.add_module(Host::<SimpleTheme>::new()));
	}
	if cwd_enabled {
		measure_elapsed("cwd", || prompt.add_module(Cwd::<SimpleTheme>::new(25, 2, false)));
	}
	if git_enabled {
		measure_elapsed("git", || prompt.add_module(Git::<SimpleTheme>::new()));
	}
	if readonly_enabled {
		measure_elapsed("readonly", || prompt.add_module(ReadOnly::<SimpleTheme>::new()));
	}
	if new_line_before_cmd_enabled {
		measure_elapsed("new_line_before_cmd", || prompt.add_module(NewLine::<SimpleTheme>::new()));
	}
	if cmd_enabled {
		measure_elapsed("cmd", || prompt.add_module(Cmd::<SimpleTheme>::new()));
	}
	println!("{}", prompt);
}

fn measure_elapsed(label: &str, mut expr: impl FnMut()) {
	let _ = label;
	#[cfg(feature = "print-module-timings")]
	let start = std::time::Instant::now();
	let result = expr();
	#[cfg(feature = "print-module-timings")]
	if env::var("POWERLINE_DEBUG_TIMINGS").unwrap_or("".to_string()) == "1" {
		println!("{} completed in {:#?}", label, std::time::Instant::now().duration_since(start));
	}
	result
}
