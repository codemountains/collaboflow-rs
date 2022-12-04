use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutTitleRequest<T>
where
    T: Serialize,
{
    pub title: T,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Serialize> PutTitleRequest<T> {
    pub fn new(title: T) -> Self {
        Self {
            title,
            _phantom: Default::default(),
        }
    }
}
