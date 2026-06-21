use bevy::math::Vec2;

pub fn triangle_area(p1: Vec2, p2: Vec2, p3: Vec2) -> f64 {
    let p1 = p1.as_dvec2();
    let p2 = p2.as_dvec2();
    let p3 = p3.as_dvec2();
    let (x1, y1) = (p2.x - p1.x, p2.y - p1.y);
    let (x2, y2) = (p3.x - p1.x, p3.y - p1.y);
    let cross = x1 * y2 - y1 * x2;
    cross.abs() * 0.5
}
#[cfg(test)]
mod tests{
    use bevy::math::Vec2;
    use crate::triangle_area::triangle_area;

    #[test]
    fn check_triangle_function() {
        assert_eq!(100.0, triangle_area(Vec2{
            x: 0.0,
            y: 0.0,
        }, Vec2 {
            x: 20.0,
            y: 0.0,
        }, Vec2{
            x: 10.0,
            y: 10.0,
        }))
    }
}
