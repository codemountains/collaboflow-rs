use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostDocumentsSimulationDetermsRequest<T>
where
    T: Serialize,
{
    pub data: T,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Serialize> PostDocumentsSimulationDetermsRequest<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            _phantom: Default::default(),
        }
    }
}
