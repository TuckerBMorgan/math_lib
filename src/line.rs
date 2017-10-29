use vec3::Vec3;

pub struct Line {
    pub a: Vec3,
    pub b: Vec3,
}

impl Line {
   #[allow(non_snake_case, dead_code)]    
    pub fn new(a: Vec3, b: Vec3) -> Line {
        Line {
            a, 
            b
        }
    }

    //we will just find the distance between the point and the line, if that distance is zero
    // then we are on the line
    #[allow(non_snake_case, dead_code)]    
    pub fn point_line_intersection(&self, point: &Vec3) -> bool {
        let mut line_direction = self.a - self.b;
        line_direction.normalize_in_place();
        let new_point = (*point) - self.a;//we derefreeance the point because the Sub Trait requires moved objects
        let dot_product = Vec3::dot_product(&new_point, &line_direction);
        let mut cloest_point_on_line = self.a + line_direction;
        cloest_point_on_line.mul_scaler_inplace(dot_product);
        return cloest_point_on_line == *point;
    }
}