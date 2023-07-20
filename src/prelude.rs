pub use crate::blocks::BlockType;
pub const CHUNK_SIZE: i32 = 16;
pub const CHUNK_ARIA: i32 = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOL: i32 = CHUNK_ARIA * CHUNK_SIZE;
pub const JIGGLE: f64 = std::f64::consts::PI - 3.;
pub const GROUND_HIGHT: f64 = 64.;
pub use crate::textures::TextureHandels;
pub const VIEW_DISTANCE: f32 = 5.;
pub use crate::world::chunk::Direction;