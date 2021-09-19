use crate::map::WorldMap;

pub struct World {
    map: WorldMap,
}

impl World {

    pub const WIDTH: u32 = 1800;
    pub const HEIGHT: u32 = Self::WIDTH / 16 * 9;
    pub const MAP_SIZE: u32 = Self::HEIGHT / 10 * 9;

    pub fn new() -> Self {
        Self {
            map: WorldMap::generate(Self::MAP_SIZE, Self::MAP_SIZE, 0.6)
        }
    }

    pub fn update(&mut self) {
    }

    pub fn draw(&self, frame: &mut [u8]) {
        let map_start_x = (Self::WIDTH - Self::MAP_SIZE) / 2;
        let map_start_y = (Self::HEIGHT - Self::MAP_SIZE) / 2;

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i as u32 % Self::WIDTH ;
            let y = i as u32 / Self::WIDTH ;

            let inside_the_box = x >= map_start_x 
                && x < map_start_x + Self::MAP_SIZE
                && y >= map_start_y
                && y < map_start_y + Self::MAP_SIZE;

            let rgba = if inside_the_box {
                let f = self.map.get(x - map_start_x, y - map_start_y);
                let f  = (f * 255.0) as u8;
                [ f, f, f, 0xff ]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}