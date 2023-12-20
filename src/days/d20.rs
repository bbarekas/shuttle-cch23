use axum::{
    routing::{get, post},
    body::Bytes,
    Router,
    response::IntoResponse};
use http::StatusCode;
use tar::Archive;
use tempfile::tempdir;
use git2::{BranchType, Commit, Repository, Tree};

const BRANCH_NAME: &str = "christmas";
const FILE_NAME: &str = "santa.txt";
const STRING_IN_FILE: &str = "COOKIE";

pub fn get_routes() -> Router {

    Router::new()
        .route("/20", get(StatusCode::OK))
        .route("/20/archive_files", post(archive_files))
        .route("/20/archive_files_size", post(archive_files_size))
        .route("/20/cookie", post(cookie))
}

async fn archive_files(body: Bytes) -> impl IntoResponse  {

    let mut tar = Archive::new(body.as_ref());
    tar.entries().unwrap().count().to_string()
}

async fn archive_files_size(body: Bytes) -> impl IntoResponse  {

    let mut tar = Archive::new(body.as_ref());
    tar.entries()
        .unwrap()
        .map(|file| file.unwrap().header().size().unwrap())
        .sum::<u64>()
        .to_string()
}

async fn cookie(body: Bytes) -> impl IntoResponse  {

    let mut tar = Archive::new(body.as_ref());
    let tmp_dir = tempdir().unwrap();
    tar.unpack(&tmp_dir).unwrap();

    let repo_path = tmp_dir.path().join(".git");

    if let Ok(repo) = Repository::open(repo_path) {
        // Get the branch reference by name (in this case, "christmas")
        if let Ok(branch) = repo.find_branch(BRANCH_NAME, BranchType::Local) {
            // Get the tip commit of the branch
            if let Ok(tip) = branch.into_reference().peel_to_commit() {
                // Start from the tip commit and traverse commit history
                if let Some(commit) = find_commit(&repo, tip) {
                    let author = commit.author();
                    let hash = commit.id();

                    let res = format!("{} {}", author.name().unwrap(), hash.to_string());
                    //println!(">>>>{}", res);
                    return res;
                }
            }
        }
    }
    "Not Found".to_string()
}

fn find_commit<'a>(repo: &Repository, commit: Commit<'a>) -> Option<Commit<'a>> {

    // Get the tree associated with the commit
    if let Ok(tree) = commit.tree() {
        if let Some(_) = find_santa_in_tree(repo, &tree, vec![]) {
            return Some(commit);
        }
    }

    // If the file is not found in this commit, traverse its parent commits
    for parent in commit.parents() {
        if let Some(found_commit) = find_commit(repo, parent) {
            return Some(found_commit);
        }
    }

    None
}

fn find_santa_in_tree<'a>(repo: &'a Repository, tree: &'a Tree<'_>, mut path: Vec<String>) -> Option<Vec<String>> {

    for entry in tree.iter() {
        let entry_name = entry.name().unwrap_or("").to_string();
        path.push(entry_name.clone());

        if let Ok(object) = entry.to_object(repo) {
            if let Some(subtree) = object.as_tree() {
                if let Some(found_path) = find_santa_in_tree(repo, subtree, path.clone()) {
                    return Some(found_path);
                }
            } else if let Some(blob) = object.as_blob() {
                if entry_name == FILE_NAME {
                    if let Ok(content) = std::str::from_utf8(blob.content()) {
                        if content.contains(STRING_IN_FILE) {
                            return Some(path);
                        }
                    }
                }
            }
        }
        path.pop();
    }

    None
}