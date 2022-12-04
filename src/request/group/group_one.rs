use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutGroupRequest<T>
where
    T: Serialize,
{
    pub group: T,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Serialize> PutGroupRequest<T> {
    pub fn new(group: T) -> Self {
        Self {
            group,
            _phantom: Default::default(),
        }
    }
}
