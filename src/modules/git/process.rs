use std::path::Path;

use crate::{modules::git::GitStats, R};

pub fn run_git(path: &Path) -> R<super::GitStats> {
	let (mut untracked, mut non_staged, mut conflicted, mut staged, mut ahead, mut behind) =
		(0, 0, 0, 0, None, None);

	let git_status = std::process::Command::new("git")
		.current_dir(path)
		.arg("status")
		.arg("--porcelain=v2")
		.arg("--branch")
		.arg("-z")
		.output()?;
	let output = String::from_utf8_lossy(&git_status.stdout).to_string();
	let output_split_by_line = output.split("\0").collect::<Vec<&str>>();
	let mut branch_name: String = "".to_string();
	let mut branch_upstream: String = "".to_string();
	for header_line in output_split_by_line.iter().filter(|line| line.starts_with("# ")) {
		let mut splits = header_line.splitn(3, " ");
		match splits.nth(1) {
			Some("branch.head") => {
				branch_name = splits.last().unwrap_or("").to_string();
			},
			Some("branch.upstream") => {
				branch_upstream = splits.last().unwrap_or("").to_string();
			},
			Some("branch.ab") => {
				let header_value = splits.last().unwrap();
				let values = header_value.splitn(2, " ").collect::<Vec<&str>>();
				if values.len() == 2 {
					ahead = *&values[0][1..].parse::<u32>().ok();
					behind = *&values[1][1..].parse::<u32>().ok();
				}
			},
			_ => {},
		}
	}

	for line in output_split_by_line.iter().filter(|line| !line.starts_with("# ")) {
		if line.is_empty() {
			continue;
		}
		let chars = format!("{:width$}", line, width = 4).chars().collect::<Vec<char>>();
		let status_part_of_line: [char; 4] = chars[0..4].try_into().unwrap_or_default();

		let status = GitStatusLine::new(status_part_of_line);

		match status.tracking {
			GitStatusTracking::Unmerged => match (status.index_status, status.working_tree_status) {
				(GitStatus::Deleted, GitStatus::Deleted | GitStatus::UpdatedButUnmerged)
				| (GitStatus::Added, GitStatus::UpdatedButUnmerged | GitStatus::Added)
				| (
					GitStatus::UpdatedButUnmerged,
					GitStatus::Deleted | GitStatus::Added | GitStatus::UpdatedButUnmerged,
				) => conflicted += 1,
				_ => {},
			},
			GitStatusTracking::Renamed | GitStatusTracking::Ordinary => {
				if status.working_tree_status != GitStatus::Unmodified {
					non_staged += 1
				}
				if status.index_status != GitStatus::Unmodified {
					staged += 1
				}
			},
			GitStatusTracking::Untracked => untracked += 1,
			_ => {},
		}
	}

	Ok(GitStats { untracked, staged, non_staged, ahead, behind, conflicted, branch_name, branch_upstream })
}

#[derive(Debug)]
struct GitStatusLine {
	tracking: GitStatusTracking,
	index_status: GitStatus,
	working_tree_status: GitStatus,
}

impl GitStatusLine {
	fn new(line: [char; 4]) -> GitStatusLine {
		GitStatusLine {
			tracking: GitStatusTracking::parse(line[0]),
			index_status: GitStatus::parse(line[2]),
			working_tree_status: GitStatus::parse(line[3]),
		}
	}
}

#[derive(Debug, PartialEq)]
enum GitStatus {
	Unmodified,
	Modified,
	Added,
	Deleted,
	Renamed,
	Copied,
	UpdatedButUnmerged,
}

impl GitStatus {
	fn parse(c: char) -> GitStatus {
		match c {
			'.' => GitStatus::Unmodified,
			'M' => GitStatus::Modified,
			'A' => GitStatus::Added,
			'D' => GitStatus::Deleted,
			'R' => GitStatus::Renamed,
			'C' => GitStatus::Copied,
			'U' => GitStatus::UpdatedButUnmerged,
			_ => GitStatus::Unmodified,
		}
	}
}

#[derive(Debug)]
enum GitStatusTracking {
	Ordinary,
	Renamed,
	Unmerged,
	Untracked,
	Ignored,
}

impl GitStatusTracking {
	fn parse(c: char) -> GitStatusTracking {
		match c {
			'1' => GitStatusTracking::Ordinary,
			'2' => GitStatusTracking::Renamed,
			'u' => GitStatusTracking::Unmerged,
			'?' => GitStatusTracking::Untracked,
			'!' => GitStatusTracking::Ignored,
			_ => GitStatusTracking::Untracked,
		}
	}
}
