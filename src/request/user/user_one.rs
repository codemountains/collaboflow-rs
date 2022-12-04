use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutUserRequest<T>
where
    T: Serialize,
{
    pub user: T,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Serialize> PutUserRequest<T> {
    pub fn new(user: T) -> Self {
        Self {
            user,
            _phantom: Default::default(),
        }
    }
}
