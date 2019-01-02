use std::ops::{Add, Sub, Neg};
use std::f32;
#[derive(Clone, Copy)]
pub struct Vector3(f32, f32, f32);

//we only need normalize, cross, op+ and op-
impl Vector3 {
    pub fn normalize(self) -> Self {
        let mut factor = ((self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)).sqrt();
        factor = 1. / factor;
        Vector3(self.0 * factor, self.1 * factor, self.2 * factor)
    }
    pub fn cross(self, other: Self) -> Self {
        let x = self.1 * other.2 - other.1 * self.2;
        let y = -(self.0 * other.2 - other.0 * self.2);
        let z = self.0 * other.1 - other.0 * self.1;
        Vector3(x, y, z)
    }
    pub fn dot(self, other: Self) -> f32 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Self::Output {
        Vector3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Self::Output {
        Vector3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector3(-self.0, -self.1, -self.2)
    }
}

pub struct Matrix((f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32));

impl Matrix {
    /// Used https://stackoverflow.com/questions/349050/calculating-a-lookat-matrix for clarification
    #[no_mangle]
    pub extern fn create_look_at(camera_pos: Vector3, camera_target: Vector3, camera_up: Vector3) -> Self {
        let z = (camera_pos - camera_target).normalize();
        let x = (Vector3::cross(camera_up, z)).normalize();
        let y = Vector3::cross(z, x);
        let mut result = Matrix((0., 0., 0., 0.), (0., 0., 0., 0.), (0., 0., 0., 0.), (0., 0., 0., 0.));
        (result.0).0 = x.0;
        (result.0).1 = y.0;
        (result.0).2 = z.0;
        (result.0).3 = 0.;
        (result.1).0 = x.1;
        (result.1).1 = y.1;
        (result.1).2 = z.1;
        (result.1).3 = 0.;
        (result.2).0 = x.2;
        (result.2).1 = y.2;
        (result.2).2 = z.2;
        (result.2).3 = 0.;
        (result.3).0 = -Vector3::dot(x, camera_pos);
        (result.3).1 = -Vector3::dot(y, camera_pos);
        (result.3).2 = -Vector3::dot(z, camera_pos);
        (result.3).3 = 1.;
        result
    }
}
