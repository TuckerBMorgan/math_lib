

use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {

    //function used for creation a new vec3
    //let a = Vec3::new(1f32, 132f, 1f32);
    //assert_eq!(a, Vec3::ONE());
    #[allow(non_snake_case, dead_code)]
    pub fn new(x: f32, y:f32, z:f32) -> Vec3 {
        Vec3 {
            x,
            y,
            z
        }
    }

    //counter part to the Add Trait
    //this does not consume the two values
    //and does it in place, saving memory
    //a.add_into_self(b); //a = a + b; 
    //where both a and b are of type Vec3
    #[allow(non_snake_case, dead_code)]
    pub fn add_into_self(&mut self, other: &Vec3) {
        self.x = other.x + self.x;
        self.y = other.y + self.y;
        self.z = other.z + self.z;
    }

    //counter part to the Sub Trait
    //this does not consume the two values
    //and does it in place, saving memory
    //a.sub_into_self(b); //a = a - b; 
    //where both a and b are of type Vec3
    #[allow(non_snake_case, dead_code)]
    pub fn sub_into_self(&mut self, other: &Vec3) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }
    
    //counter part to the Mul Trait
    //this does not consume the two values
    //and does it in place, saving memory
    //a.mul_into_self(b); //a = a * b; 
    //where both a and b are of type Vec3
    #[allow(non_snake_case, dead_code)]
    pub fn mul_into_self(&mut self, other: &Vec3) {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
    }

    //let mut a = Vec3::new(132, 1f32, 1f32).mul_scaler_inplace(10f32);    
    //assert_eq!(a, Vec3::new(10f32, 10f32, 10f32));    
    #[allow(non_snake_case, dead_code)]
    pub fn mul_scaler_inplace(&mut self, scaler: f32) {
        self.x = self.x * scaler;
        self.y = self.y * scaler;
        self.z = self.z * scaler;
    }

    //let a = Vec3::new(1f32, 1f32, 1f32);
    //let mut b = a.mut_scaler_into_new();
    #[allow(non_snake_case, dead_code)]
    pub fn mul_scaler_into_new(& self, scaler: f32) -> Vec3{
        Vec3 {
            x: self.x * scaler,
            y: self.y * scaler,
            z: self.z * scaler
        }
    }

    //this returns of the length of the vec3
    //a^ 2 + b ^ 2 + c ^ 2
    #[allow(non_snake_case, dead_code)]
    pub fn length(&self) -> f32 {
        let mut length = self.x * self.x;
        length = length + (self.y * self.y);
        length = length + (self.z * self.z);
        return f32::sqrt(length);
    }
    
    //will normalize the vec in place, so it does not allocate a new vec3
    #[allow(non_snake_case, dead_code)]
    pub fn normalize_in_place(&mut self)  {
        let length = self.length();
        if length != 0f32 {
            self.x = self.x / length;
            self.y = self.y / length;
            self.z = self.z / length;
        }
        else {
            self.x = 1f32;
            self.y = 1f32;
            self.z = 1f32;
        }
    }
    
    //will normalize the vec into a new vector so it will cause an allocation
    #[allow(non_snake_case, dead_code)]
    pub fn normalize_into_new(&self) -> Vec3 {
        let length = self.length();

        if length != 0f32 {
           return Vec3 {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length
            }
        }

        Vec3::ONE()
    }

    //returns the dot product between two vec3s
    #[allow(non_snake_case, dead_code)]
    pub fn dot_product_with_another(&self, other: &Vec3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * self.z)
    }

    //returns the dot product between two vec3s
    #[allow(non_snake_case, dead_code)]
    pub fn dot_product(a: &Vec3, b: &Vec3) -> f32 {
        (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
    }

    //returns the cross product between two vec3s
    #[allow(non_snake_case, dead_code)]
    pub fn cross_product(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    //helper function for common Zero vector
    #[allow(non_snake_case, dead_code)]
    pub fn ZERO() -> Vec3 {
        Vec3 {
            x: 0f32,
            y: 0f32,
            z: 0f32
        }
    }

    //helper function for common One Vector
    #[allow(non_snake_case, dead_code)]
    pub fn ONE() -> Vec3 {
        Vec3 {
            x: 1f32,
            y: 1f32,
            z: 1f32
        }
    }

    //a function that returns weither two vectors are orthogonal to each other
    #[allow(non_snake_case, dead_code)]
    pub fn are_orthogonal(a: &Vec3, b: &Vec3) -> bool {
        return Vec3::dot_product(a, b) == 0f32;
    }
    
    //returns the vector projection of a onto b
    #[allow(non_snake_case, dead_code)]
    pub fn project_a_onto_b_vector_(a: &Vec3, b: &Vec3) -> Vec3 {
        let dot = Vec3::dot_product(a, b);
        let scaler = Vec3::length(b) * Vec3::length(b);
        let result = dot / scaler;
        return b.mul_scaler_into_new(result);
    }

    //a predicate used to determine if two vectors are colinear to each other
    #[allow(non_snake_case, dead_code)]
    pub fn are_colinear(a: &Vec3, b: &Vec3) -> bool {
        return Vec3::cross_product(a, b) == Vec3::ZERO();
    }
    
    //simple helper function for the distance between any two vec3s, uses sqrt, so be careful
    //let dis : f32 = Vec3::distance_between_two(Vec3::ONE(), Vec3::ZERO());
    //assert_eq!(dis, 1f32);
    #[allow(non_snake_case, dead_code)]
    pub fn distance_between_two(a: &Vec3, b: &Vec3) -> f32 {
        f32::sqrt((b.x - a.x) + (b.y - a.y) + (b.z - a.z))
    }

    //returns the angle between any two vec3s
    #[allow(non_snake_case, dead_code)]    
    pub fn angle_between_two_vectors(a: &Vec3, b: &Vec3) -> f32 {
        if *a == Vec3::ZERO() {
            return 0f32;
        }

        if *b == Vec3::ONE() {
            return 0f32;
        }

        let dot = Vec3::dot_product(a, b);
        let length_a = a.length();
        let length_b = b.length();
        let pre_cos = dot / (length_a * length_b);
        return pre_cos;
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x : self.x - other.x,
            y : self.y - other.y,
            z : self.z - other.z
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x : self.x * other.x,
            y : self.y * other.y,
            z : self.z * other.z
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3 ) -> bool {
        let dis = (other.x - self.x) + (other.y - self.y) + (other.z - self.z);
        let dis = dis * dis;
        return dis < 0.0001f32;//we cannot relay on regualr equal because of floats being slightly off
    }
}