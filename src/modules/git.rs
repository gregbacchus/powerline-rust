use std::{env, marker::PhantomData, path, path::PathBuf};

#[cfg(feature = "libgit")]
use libgit as internal;
#[cfg(not(feature = "libgit"))]
use process as internal;

use crate::{terminal::Color, Segment, R};

use super::Module;

#[cfg(feature = "libgit")]
mod libgit;
#[cfg(not(feature = "libgit"))]
mod process;

pub struct Git<S> {
	scheme: PhantomData<S>,
}

#[derive(Clone)]
pub struct GitStats {
	pub untracked: u32,
	pub conflicted: u32,
	pub non_staged: u32,
	pub ahead: Option<u32>,
	pub behind: Option<u32>,
	pub staged: u32,
	pub branch_name: String,
	pub branch_upstream: String,
}

pub trait GitScheme {
	const GIT_AHEAD_BEHIND_BG: Color;
	const GIT_AHEAD_BEHIND_FG: Color;
	const GIT_CONFLICTED_BG: Color;
	const GIT_CONFLICTED_FG: Color;
	const GIT_NOT_STAGED_BG: Color;
	const GIT_NOT_STAGED_FG: Color;
	const GIT_REPO_CLEAN_BG: Color;
	const GIT_REPO_CLEAN_FG: Color;
	const GIT_REPO_DIRTY_BG: Color;
	const GIT_REPO_DIRTY_FG: Color;
	const GIT_REPO_ERROR_BG: Color;
	const GIT_REPO_ERROR_FG: Color;
	const GIT_REPO_NO_UPSTREAM_BG: Color;
	const GIT_REPO_NO_UPSTREAM_FG: Color;
	const GIT_STAGED_BG: Color;
	const GIT_STAGED_FG: Color;
	const GIT_UNTRACKED_BG: Color;
	const GIT_UNTRACKED_FG: Color;
}

impl<S: GitScheme> Git<S> {
	pub fn new() -> Git<S> {
		Git { scheme: PhantomData }
	}

	pub fn get_git_data(&mut self, path: PathBuf) -> R<GitStats> {
		internal::run_git(&path)
	}
}

impl GitStats {
	pub fn is_dirty(&self) -> bool {
		(self.untracked + self.conflicted + self.staged + self.non_staged) > 0
	}
}

fn find_git_dir() -> Option<path::PathBuf> {
	let mut git_dir = env::current_dir().ok()?;
	loop {
		git_dir.push(".git/");

		if git_dir.exists() {
			git_dir.pop();
			return Some(git_dir);
		}
		git_dir.pop();

		if !git_dir.pop() {
			return None;
		}
	}
}

impl<S: GitScheme> Module for Git<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) {
		let git_dir = match find_git_dir() {
			Some(dir) => dir,
			_ => return,
		};

		let stats = self.get_git_data(git_dir);
		stats
			.map(|git_stats| {
				let (branch_sym, branch_fg, branch_bg) = if git_stats.is_dirty() {
					("󰘬".to_string(), S::GIT_REPO_DIRTY_FG, S::GIT_REPO_DIRTY_BG)
				} else if git_stats.branch_upstream.is_empty() {
					("󰽤".to_string(), S::GIT_REPO_NO_UPSTREAM_FG, S::GIT_REPO_NO_UPSTREAM_BG)
				} else {
					("󰘬".to_string(), S::GIT_REPO_CLEAN_FG, S::GIT_REPO_CLEAN_BG)
				};

				segments.push(Segment::simple(
					format!(" {} {} ", branch_sym, git_stats.branch_name),
					branch_fg,
					branch_bg,
				));

				let mut add_elem = |count, symbol, fg, bg| match count {
					1 => segments.push(Segment::simple(format!(" {}  ", symbol), fg, bg)),
					0 => (),
					_ => segments.push(Segment::simple(format!(" {} {} ", symbol, count), fg, bg)),
				};

				if let Some(ahead) = git_stats.ahead {
					if ahead > 0 {
						add_elem(ahead, '\u{F0AA}', S::GIT_AHEAD_BEHIND_FG, S::GIT_AHEAD_BEHIND_BG)
					}
				}
				if let Some(behind) = git_stats.behind {
					if behind > 0 {
						add_elem(behind, '\u{F0AB}', S::GIT_AHEAD_BEHIND_FG, S::GIT_AHEAD_BEHIND_BG);
					}
				}

				add_elem(git_stats.staged, '\u{F00C}', S::GIT_STAGED_FG, S::GIT_STAGED_BG);
				add_elem(git_stats.non_staged, '\u{F040}', S::GIT_NOT_STAGED_FG, S::GIT_NOT_STAGED_BG);
				add_elem(git_stats.untracked, '\u{F067}', S::GIT_UNTRACKED_FG, S::GIT_UNTRACKED_BG);
				add_elem(git_stats.conflicted, '\u{273C}', S::GIT_CONFLICTED_FG, S::GIT_CONFLICTED_BG);
			})
			.map_err(|error| {
				segments.push(Segment::simple(
					format!("git error: {:?}", error),
					S::GIT_REPO_ERROR_FG,
					S::GIT_REPO_ERROR_BG,
				))
			})
			.unwrap_or_default();
	}
}
