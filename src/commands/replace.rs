pub fn replace(deadname: &str, newname: &str, repo: &str) {
    println!(
        "I replace instances of {:?} with {:?} within a repo at {:?}.",
        deadname, newname, repo
    );
}
