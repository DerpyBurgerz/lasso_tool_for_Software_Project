use bevy::math::Vec2;

#[derive(Clone, Debug)]
pub struct CumulativeTriangleRoutine {
    pub first: usize,
    pub middle: usize,
    pub last: usize,
    pub temp_area: f64,
    pub threshhold_area: f64,
}

impl Default for CumulativeTriangleRoutine {
    fn default() -> Self {
        CumulativeTriangleRoutine{
            first: 0,
            middle: 1,
            last: 2,
            temp_area: 0.0,
            threshhold_area: 0.0,
        }
    }
}

pub fn cumulative_triangle_routine_step(points: Vec<Vec2>, data: CumulativeTriangleRoutine) -> (Vec<Vec2>, CumulativeTriangleRoutine) {

    todo!()
}
