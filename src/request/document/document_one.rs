use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PutDocumentStatusRequest<T>
where
    T: Serialize,
{
    pub document: T,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Serialize> PutDocumentStatusRequest<T> {
    pub fn new(document: T) -> Self {
        Self {
            document,
            _phantom: Default::default(),
        }
    }
}
