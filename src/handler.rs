use anyhow::Result;

pub trait Handler<T> {
    fn handle(&self, input: T) -> Result<()>;
}
