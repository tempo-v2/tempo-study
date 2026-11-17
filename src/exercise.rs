use serde::{Deserialize, Serialize};
use tempo_types::prelude::*;


/// An exercise that can be presented to the user.
/// Contains the target (what the user is expected to produce)
/// and the definition of the exercise itself. These are very generic
/// and can be adapted to different types of exercises.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise<D> {
    pub target: TargetId,
    pub version: ContentVersion,
    pub definition: D,
}
