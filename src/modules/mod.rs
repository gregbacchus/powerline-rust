use crate::powerline::Segment;

mod cmd;
mod cwd;
mod exit_code;
mod git;
mod host;
mod pyvenv;
mod readonly;
mod user;
mod fish;
mod exec_time;

#[cfg(feature = "time")]
mod time;

pub use cmd::{Cmd, CmdScheme};
pub use cwd::{Cwd, CwdScheme};
pub use exit_code::{ExitCode, ExitCodeScheme};
pub use git::{Git, GitScheme};
pub use host::{Host, HostScheme};
pub use pyvenv::{PyVenv, PyVenvScheme};
pub use readonly::{ReadOnly, ReadOnlyScheme};
pub use user::{User, UserScheme};
pub use fish::{Fish, FishScheme};
pub use exec_time::{ExecTime, ExecTimeScheme};

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
