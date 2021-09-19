use noise::{NoiseFn, Perlin};

pub struct WorldMap {
    width: u32,
    _height: u32,
    values: Vec<f64>,
}

impl WorldMap {
    pub fn generate(width: u32, height: u32, mut scale: f64) -> WorldMap {
        let perlin = Perlin::new();

        if scale <= 0.0 {
            scale = 0.0001;
        }

        let mut values = vec![];
        for y in 0..height {
            for x in 0..width {
                let x = x as f64 / scale;
                let y = y as f64 / scale;

                values.push(perlin.get([x as f64, y as f64]));
            }
        }

        WorldMap {
            width,
            _height: height,
            values,
        }
    }

    pub fn _width(&self) -> u32 {
        self.width
    }

    pub fn _height(&self) -> u32 {
        self._height
    }

    pub fn get(&self, x: u32, y: u32) -> f64 {
        self.values[(y * self.width + x) as usize]
    }
}
