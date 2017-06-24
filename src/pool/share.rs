use std::sync::Arc;

pub struct StringShare {
    arc: Arc<String>,
}

impl Borrow<&str> for StringShare {
    fn borrow(&self) -> &StringShare {
    }
}

