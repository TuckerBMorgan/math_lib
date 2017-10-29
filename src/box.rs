use vec3::Vec3;

//called BoxShape because "Box" is a reserved keyword in rust
pub struct BoxShape {
    pub center: Vec3,
    pub width: f32,
    pub height: f32
}


impl BoxShape {
    
    pub fn new(center: Vec3, width: f32, height: f32) -> BoxShape {
        BoxShape {
            center, 
            width
            height,
        }
    }

    

}