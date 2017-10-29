
mod vec3;
mod line;
mod plane;

#[cfg(test)]
mod tests {
    use vec3::Vec3;
    use line::Line;
    use plane::Plane;

    #[test]
    fn vec3_add_test() {
        let a = Vec3::ZERO();
        let b = Vec3::ONE();
        
        //basic addition
        let c = a + b;
        assert_eq!(c, Vec3::ONE());
        
        //commutative property
        let c = b + a;
        assert_eq!(c, Vec3::ONE());

        //identity
        let c = b + Vec3::ZERO();
        assert_eq!(c, b);
    }

    #[test]
    fn vec3_sub_test() {
        let a = Vec3::new(5f32, 10.0f32, 1.5f32);
        let b = Vec3::new(4f32, 7.3f32, 0.5f32);

        
        //basic subtraction
        let c = a - b;
        assert_eq!(c, Vec3::new(1f32, 10.0f32 - 7.3f32, 1f32));

        let c = a - Vec3::ZERO();
        assert_eq!(c, a);
    }

    #[test]
    fn vec3_mul_test() {
        let a = Vec3::new(2f32, 1f32, 3f32);
        let b = Vec3::new(3f32, 5f32, 9f32);

        let local_test_var = Vec3::new(6f32, 5f32, 27f32);
        
        //basic multiplication 
        let c = a * b;
        assert_eq!(c, local_test_var);

        //commutative property
        let c = b * a;
        assert_eq!(c, local_test_var);

        //zeroing out
        let c = a * Vec3::ZERO();
        assert_eq!(c, Vec3::ZERO());


        //Identify function
        let c = a * Vec3::ONE();
        assert_eq!(c, a);
    }

    #[test]
    fn point_line_test() {
        let start_point = Vec3::ZERO();
        let end_point = Vec3::ONE();

        let mut line = Line::new(start_point, end_point);

        //does this point lay along this line
        assert_eq!(line.point_line_intersection(&Vec3::new(0.5f32,0.5f32, 0.5f32)), true);


        let start_point = Vec3::ZERO();
        let end_point = Vec3::new(10.0f32, 5.4f32, 1.3f32);
        line.a = start_point;
        line.b = end_point;

        assert_eq!(line.point_line_intersection(&Vec3::ONE()), false);
    }

    #[test] 
    fn misc_vec3_tests() {
        //angle between two vectors
        //a 90 degree angle
        let a = Vec3::new(1f32, 0f32, 0f32);
        let b = Vec3::new(0f32, 1f32, 0f32);
        let angle = Vec3::angle_between_two_vectors(&a, &b);
        assert_eq!(angle, 0f32);

        let b =  Vec3::new(0.5f32, 0.5f32, 0f32);

        let angle = Vec3::angle_between_two_vectors(&a, &b);
        println!("{}", angle);
    }

    #[test] 
    fn plane_tests() {
        let a = Vec3::new(1f32, 0f32, 0f32);
        let b = Vec3::new(0f32, 0f32, 1f32);
        let c = Vec3::new(0f32, 0f32, 0f32);

        let plane = Plane::new(a, b, c);
        println!("{:?}", plane.normal_of_plane());
    }
}


