use std::process::Command;

fn main()
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
    Ok(());
}

fn git_add()
{
    let git_addd = Command::new("git")
        .arg("add")
        .arg(".")
        .spawn()?;
}

fn git_commit()
{
    let git_addd = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("'random commit'")
        .spawn()?;
}

fn git_push()
{
    let git_addd = Command::new("git")
        .arg("push")
        .spawn()?;
}
