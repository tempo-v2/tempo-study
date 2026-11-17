use crate::{progress::Progress, study::StudyError};

pub type SubmitResult<S, P, F> = Result<Progress<S, P, F>, StudyError>;
