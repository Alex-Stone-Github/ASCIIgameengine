#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }
    pub fn normalise(&mut self) {
        let length: f32 = (self.x*self.x+self.y*self.y+self.z*self.z).sqrt();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }
    pub fn opposite(&self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}
pub struct Face {
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,
    pub normal: Vector3,
}
impl Face {
}
