use serde::{Deserialize, Serialize};
use tempo_types::prelude::*;

use crate::grader::Grade;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptStatus {
    Active,
    Complete,
    Abandoned,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attempt<S, P> {
    pub id: AttemptId,
    pub dataset: DatasetId,
    pub target: TargetId,
    pub content_version: ContentVersion,
    pub revision: Revision,
    pub state: S,
    pub prompt: Option<P>,
    pub terminal: Option<Grade>,
    pub status: AttemptStatus,
}
