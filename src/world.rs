use imgui;

use crate::map::WorldMap;

pub struct World {
    map_width: u32,
    map_height: u32,
    noise_scale: f64,
    map: WorldMap,
}

impl World {
    pub const WINDOW_WIDTH: u32 = 1800;
    pub const WINDOW_HEIGHT: u32 = Self::WINDOW_WIDTH / 16 * 9;

    pub fn new() -> Self {
        let map_width = Self::WINDOW_HEIGHT / 10 * 9;
        let map_height = Self::WINDOW_HEIGHT / 10 * 9;
        let noise_scale = 0.6;
        Self {
            map_width,
            map_height,
            noise_scale,
            map: WorldMap::generate(map_width, map_height, 0.6),
        }
    }

    pub fn update(&mut self) {}

    pub fn update_on_gui(&mut self, ui: &imgui::Ui) {
        imgui::Window::new(imgui::im_str!("World"))
            .size([300.0, 110.0], imgui::Condition::FirstUseEver)
            .build(ui, || {
                let mut updated = false;

                updated = updated
                    || imgui::Slider::new(imgui::im_str!("Map Width"))
                        .range(0..=World::WINDOW_WIDTH)
                        .build(ui, &mut self.map_width);

                updated = updated
                    || imgui::Slider::new(imgui::im_str!("Map Height"))
                        .range(0..=World::WINDOW_HEIGHT)
                        .build(ui, &mut self.map_height);

                updated = updated
                    || imgui::Slider::new(imgui::im_str!("Noise Scale"))
                        .range(0.0..=100.0)
                        .build(ui, &mut self.noise_scale);

                if updated {
                    self.map =
                        WorldMap::generate(self.map_width, self.map_height, self.noise_scale);
                }
            });
    }

    pub fn draw(&self, frame: &mut [u8]) {
        let map_start_x = (Self::WINDOW_WIDTH - self.map_width) / 2;
        let map_start_y = (Self::WINDOW_HEIGHT - self.map_height) / 2;

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i as u32 % Self::WINDOW_WIDTH;
            let y = i as u32 / Self::WINDOW_WIDTH;

            let inside_the_box = x >= map_start_x
                && x < map_start_x + self.map_width
                && y >= map_start_y
                && y < map_start_y + self.map_height;

            let rgba = if inside_the_box {
                let f = self.map.get(x - map_start_x, y - map_start_y);
                let f = (f * 255.0) as u8;
                [f, f, f, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
