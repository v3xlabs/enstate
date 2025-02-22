use async_trait::async_trait;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Factory<T>: Send + Sync {
    fn get_instance(&self) -> T;
}

pub struct SimpleFactory<T: Send + Sync + Clone>(T);

impl<T: Send + Sync + Clone> Factory<T> for SimpleFactory<T> {
    fn get_instance(&self) -> T {
        self.0.clone()
    }
}

impl<T: Send + Sync + Clone> From<T> for SimpleFactory<T> {
    fn from(value: T) -> Self {
        SimpleFactory(value)
    }
}
