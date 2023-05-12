use git2::Status;

pub fn test_repo(dir: &str) {
    let repo = git2::Repository::open(dir).unwrap();
    let statuses = repo.statuses(None).unwrap();

    let mut collected_statuses = Vec::with_capacity(statuses.len());

    for i in 0..statuses.len() {
        let status_entry = statuses.get(i).unwrap();
        let status = status_entry.status();
        let path = status_entry.path();
        let path = if path.is_some() { path.unwrap().to_owned() }
        else {"invalid-utf8-name".to_string()};
        collected_statuses.push((path, status));
    }

    filter_index_statuses(&collected_statuses);
}

pub fn filter_index_statuses(statuses: &Vec<(String, Status)>) {
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
        println!("No dirty files in the working directory");
    } else {
        println!("Dirty files in the working directory:");
        for status in new { println!("New: {}", status.0); }
        for status in modified { println!("Modified: {}", status.0); }
        for status in deleted { println!("Deleted: {}", status.0); }
        for status in typechange { println!("Typechange: {}", status.0); }
        for status in renamed { println!("Renamed: {}", status.0); }
    }
}
