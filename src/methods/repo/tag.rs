use std::sync::Arc;

use askama::Template;
use axum::{extract::Query, response::Response, Extension};
use serde::Deserialize;

use crate::{
    git::DetailedTag,
    into_response,
    methods::repo::{Repository, RepositoryPath, Result},
    Git,
};

#[derive(Deserialize)]
pub struct UriQuery {
    #[serde(rename = "h")]
    name: String,
}

#[derive(Template)]
#[template(path = "repo/tag.html")]
pub struct View {
    repo: Repository,
    tag: DetailedTag,
}

pub async fn handle(
    Extension(repo): Extension<Repository>,
    Extension(RepositoryPath(repository_path)): Extension<RepositoryPath>,
    Extension(git): Extension<Arc<Git>>,
    Query(query): Query<UriQuery>,
) -> Result<Response> {
    let open_repo = git.repo(repository_path).await?;
    let tag = open_repo.tag_info(&query.name).await?;

    Ok(into_response(&View { repo, tag }))
}
