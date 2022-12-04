use serde::Serialize;

pub struct PostDocumentsSearchRequest<T>
where
    T: Serialize,
{
    pub data: T,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Serialize> PostDocumentsSearchRequest<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            _phantom: Default::default(),
        }
    }
}
