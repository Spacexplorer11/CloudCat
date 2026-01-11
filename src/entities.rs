pub mod cat;
pub mod cloud;
pub mod floor;
pub mod umbrella;

pub trait Animation {
    async fn draw(&mut self) -> (f32, i32);
}
