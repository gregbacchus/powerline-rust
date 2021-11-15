use std::path::Path;

use git2::{Branch, BranchType, ObjectType, Repository, Status, StatusOptions, StatusShow};

use crate::{modules::git::GitStats, R};
use std::process::Command;

pub fn run_git(path: &Path) -> R<super::GitStats> {
    let (mut untracked, mut non_staged, mut conflicted, mut staged, mut ahead, mut behind) =
        (0, 0, 0, 0, 0, 0);

    let git_status = std::process::Command::new("git")
        .current_dir(path)
        .arg("status")
        .arg("--porcelain=v2")
        .arg("--branch")
        .arg("-z")
        .output()?;
    let output = String::from_utf8_lossy(&git_status.stdout).to_string();
    let split_output = output.split("\0").collect::<Vec<&str>>();
    let branch_name = split_output[1].split_at(14).1.to_string();
    for line in split_output.split_at(4).1 {
        let chars = line.chars().collect::<Vec<char>>();
        if chars.is_empty() {
            continue
        }
        // https://git-scm.com/docs/git-status#_porcelain_format_version_2
        // If first char is a '?', we're untracked
        if chars[0] == '?' {
            untracked += 1;
            continue;
        }
        match chars[2] {
            'D' | 'M' => { staged += 1 }
            _ => {}
        }
        match chars[3] {
            'D' | 'M' => { non_staged += 1 }
            _ => {}
        }
    }

    Ok(GitStats {
        untracked,
        staged,
        non_staged,
        ahead: ahead as u32,
        behind: behind as u32,
        conflicted,
        branch_name,
    })
}
