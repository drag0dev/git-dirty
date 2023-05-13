use anyhow::{Result, Context};
use colored::Colorize;
use git2::{Status, RepositoryState};

pub fn test_repo(dir: &str) -> Result<()> {
    let repo = git2::Repository::open(dir)
        .context("reading git repository")?;
    let statuses = repo.statuses(None)
        .context("reading statuses")?;

    let mut collected_statuses = Vec::with_capacity(statuses.len());

    for i in 0..statuses.len() {
        let status_entry = statuses.get(i).unwrap();
        let status = status_entry.status();
        let path = status_entry.path();
        let path = if path.is_some() { path.unwrap().to_owned() }
        else {"invalid-utf8-name".to_string()};
        collected_statuses.push((path, status));
    }

    check_state(repo.state());
    filter_index_statuses(&collected_statuses);
    filter_working_directory_statuses(&collected_statuses);
    count_ignored(&collected_statuses);
    Ok(())
}

pub fn filter_working_directory_statuses(statuses: &Vec<(String, Status)>) {
    let mut new = Vec::new();
    let mut modified = Vec::new();
    let mut deleted = Vec::new();
    let mut typechange = Vec::new();
    let mut renamed = Vec::new();
    for status in statuses {
        match status.1 {
            Status::WT_NEW => new.push(status),
            Status::WT_MODIFIED => modified.push(status),
            Status::WT_DELETED => deleted.push(status),
            Status::WT_TYPECHANGE => typechange.push(status),
            Status::WT_RENAMED => renamed.push(status),
            _ => {}
        }
    }

    if new.len() == 0 && modified.len() == 0 && deleted.len() == 0 &&
        typechange.len() == 0 && renamed.len() == 0 {
        println!("{}", "No dirty files in the working directory".green());
    } else {
        println!("{}", "Dirty files in the working directory:".red());
        for status in new { println!("\tNew: {}", status.0); }
        for status in modified { println!("\tModified: {}", status.0); }
        for status in deleted { println!("\tDeleted: {}", status.0); }
        for status in typechange { println!("\tTypechange: {}", status.0); }
        for status in renamed { println!("\tRenamed: {}", status.0); }
    }
}

pub fn filter_index_statuses(statuses: &Vec<(String, Status)>) {
    let mut new = Vec::new();
    let mut modified = Vec::new();
    let mut deleted = Vec::new();
    let mut typechange = Vec::new();
    let mut renamed = Vec::new();
    for status in statuses {
        match status.1 {
            Status::INDEX_NEW => new.push(status),
            Status::INDEX_MODIFIED => modified.push(status),
            Status::INDEX_DELETED => deleted.push(status),
            Status::INDEX_TYPECHANGE => typechange.push(status),
            Status::INDEX_RENAMED => renamed.push(status),
            _ => {}
        }
    }

    if new.len() == 0 && modified.len() == 0 && deleted.len() == 0 &&
        typechange.len() == 0 && renamed.len() == 0 {
        println!("{}", "No dirty files in the index".green());
    } else {
        println!("{}", "Dirty files in the index:".red());
        for status in new { println!("\tNew: {}", status.0); }
        for status in modified { println!("\tModified: {}", status.0); }
        for status in deleted { println!("\tDeleted: {}", status.0); }
        for status in typechange { println!("\tTypechange: {}", status.0); }
        for status in renamed { println!("\tRenamed: {}", status.0); }
    }
}

pub fn count_ignored(statuses: &Vec<(String, Status)>) {
    let mut count = 0;
    for status in statuses {
        if status.1 == Status::IGNORED { count += 1; }
    }

    if count != 0 {
        println!("{}: {}", "Ignored files".yellow(), count);
    }
}

pub fn check_state(state: RepositoryState) {
    if state != RepositoryState::Clean {
        println!("{} {} {}",
            "-- REPOSITORY STATE:".red().bold(),
            format!("{:?}", state).red().bold(),
            "--".red().bold());
    }
}
