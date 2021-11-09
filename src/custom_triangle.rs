use sfml::graphics::CustomShapePoints;
use sfml::system::Vector2f;

#[derive(Clone, Copy)]
pub struct TriangleShape;

impl CustomShapePoints for TriangleShape {
    fn point_count(&self) -> u32 {
        3
    }

    fn point(&self, point: u32) -> Vector2f {
        match point {
            0 => Vector2f { x: 20., y: 580. },
            1 => Vector2f { x: 400., y: 20. },
            2 => Vector2f { x: 780., y: 580. },
            p => panic!("Non-existent point: {}", p),
        }
    }
}