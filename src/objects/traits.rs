use std::any::Any;

/// Define the type id of the object.
pub trait TypeId: Any {
    /// Rertuns the type id.
    fn type_id(&self) -> u32;
}

/// Serialize the data.
pub trait Serialize {
    /// Serialize the data.
    fn serialize(&self) -> Vec<u8>;
}
