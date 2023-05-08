use anyhow::Result;

pub fn test_repo(dir: &str) -> Result<()> {
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

    println!("{:?}", collected_statuses);
    Ok(())
}
