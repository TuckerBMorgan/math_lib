use vec3::Vec3;

pub struct Plane {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3
}

impl Plane {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Plane {
        Plane {
            a,
            b,
            c
        }
    }

    pub fn normal_of_plane(&self) -> Vec3 {
        let dir_1 = self.b - self.a;
        let dir_2 = self.c - self.a;
        let cross = dir_1.cross_product(&dir_2);
        return cross;
    }
}