use num::Num;
use na::{
    base::dimension::U2,
    geometry::{Isometry2, Translation, UnitComplex},
};

pub type Translation2<T> = Translation<T, U2>;

#[derive(Copy, Clone, Debug)]
pub struct Cuboid<T> {
    pub width: T, 
    pub height: T,
}

impl<T> Cuboid<T> {
    pub fn new(width: T, height: T) -> Cuboid<T> {
        Cuboid {
            width,
            height
        }   
    }
}

pub struct AABB<T: na::base::Scalar> {
    pub rect: Cuboid<T>,
    pub translation: Translation2<T>
}

impl<T: na::base::Scalar + Num> AABB<T> {
    pub fn translate(&mut self, translation: na::Vector2<T>) {
        self.translation.vector.x = self.translation.vector.x + translation.x;
        self.translation.vector.y = self.translation.vector.y + translation.y;
    }

    pub fn get_translation(&self) -> na::Vector2<T> {
        self.translation.vector
    }
}

pub struct BoundingBox<T: na::Real> {
    pub rect: Cuboid<T>,
    pub transformation: Isometry2<T>
}

impl<T: na::Real> BoundingBox<T> {
    pub fn translate(&mut self, translation: na::Vector2<T>) {
        self.transformation.append_translation_mut(&Translation::from(translation));
    }

    pub fn rotate(&mut self, angle: T) {
        let rotation = UnitComplex::from_angle(angle);
        self.transformation.append_rotation_mut(&rotation);
    }

    pub fn get_translation(&self) -> na::Vector2<T> {
        self.transformation.translation.vector
    }

    pub fn get_rotation(&self) -> T {
        self.transformation.rotation.angle()
    }
}