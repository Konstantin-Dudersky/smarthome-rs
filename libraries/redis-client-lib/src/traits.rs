/// Ключ для сохранения в базе данных
pub trait GetKey {
    fn key(&self) -> String;
}
