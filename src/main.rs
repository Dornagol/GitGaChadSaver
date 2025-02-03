use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let is_git_repo = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()?;

    if !is_git_repo.status.success()
    {
        eprintln!("T'es pas dans un repo git 👉🙉");
        return Ok(());
    }
    git_add()?;
    git_commit()?;
    git_push()?;
    println!("Repo git bien push 🗿");
    Ok(())
}

fn git_add() -> Result<(), Box<dyn std::error::Error>>
{
    let status = Command::new("git")
        .arg("add")
        .arg(".")
        .status()?;
    
    if !status.success() {
        return Err("Echec de 'git add'👉🙉".into());
    }
    Ok(())
}

fn git_commit() -> Result<(), Box<dyn std::error::Error>>
{
    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("random commit")
        .status()?;

    if !status.success() {
        return Err("Echec de 'git commit'👉🙉".into());
    }
    Ok(())
}

fn git_push() -> Result<(), Box<dyn std::error::Error>>
{
    let status = Command::new("git")
        .arg("push")
        .status()?;

    if !status.success() {
        return Err("Echec de 'git push'👉🙉".into());
    }
    Ok(())
}
