use crate::{
    Availability,
    start::{StartError, Started},
};

pub type NextResult<D, S, P, E> = Result<Availability<Started<D, S, P>>, StartError<E>>;
