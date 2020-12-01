use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[allow(dead_code)]
    pub fn randomize(origin: Vec2, spread: f32) -> Vec2 {
        let mut x = origin.x;
        let mut y = origin.y;
        x += rand::thread_rng().gen_range(-spread, spread);
        y += rand::thread_rng().gen_range(-spread, spread);
        Vec2 { x, y }
    }

    pub fn dot(&self, rhs: &Vec2) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
    pub fn right() -> Vec2 {
        Vec2 { x: 1.0, y: 0.0 }
    }

    pub fn left() -> Vec2 {
        Vec2 { x: -1.0, y: 0.0 }
    }

    pub fn up() -> Vec2 {
        Vec2 { x: 0.0, y: -1.0 }
    }

    pub fn down() -> Vec2 {
        Vec2 { x: 0.0, y: 1.0 }
    }
    #[allow(dead_code)]
    pub fn normalzed(&self) -> Vec2 {
        let r: f32 = 1.0 / self.length();
        Vec2 {
            x: self.x * r,
            y: self.y * r,
        }
    }
    #[allow(dead_code)]
    pub fn direction(from: Vec2, to: Vec2) -> Vec2 {
        let diff = to - from;
        diff.normalzed()
    }

    pub fn distance(from: Vec2, to: Vec2) -> f32 {
        let diff = to - from;
        diff.length()
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
