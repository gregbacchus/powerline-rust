use crate::powerline::Segment;

mod cmd;
mod cwd;
mod exec_time;
mod exit_code;
mod fish;
mod git;
mod host;
mod new_line;
mod pyvenv;
mod readonly;
mod user;

#[cfg(feature = "time")]
mod time;

pub use cmd::{Cmd, CmdScheme};
pub use cwd::{Cwd, CwdScheme};
pub use exec_time::{ExecTime, ExecTimeScheme};
pub use exit_code::{ExitCode, ExitCodeScheme};
pub use fish::{Fish, FishScheme};
pub use git::{Git, GitScheme};
pub use host::{Host, HostScheme};
pub use new_line::{NewLine, NewLineScheme};
pub use pyvenv::{PyVenv, PyVenvScheme};
pub use readonly::{ReadOnly, ReadOnlyScheme};
pub use user::{User, UserScheme};

#[cfg(feature = "time")]
pub use time::{Time, TimeScheme};

pub trait Module: Sized {
	fn append_segments(&mut self, segments: &mut Vec<Segment>);

	#[inline]
	fn into_segments(mut self) -> Vec<Segment> {
		self.get_segments()
	}

	#[inline]
	fn get_segments(&mut self) -> Vec<Segment> {
		let mut vec = Vec::new();

		self.append_segments(&mut vec);
		vec
	}
}
