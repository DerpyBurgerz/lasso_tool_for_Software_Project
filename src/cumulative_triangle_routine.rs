use bevy::math::Vec2;
use crate::triangle_area::triangle_area;

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

pub fn cumulative_triangle_routine_step(points: &mut Vec<Vec2>, data: CumulativeTriangleRoutine) -> CumulativeTriangleRoutine {
    // add input[0] to result array
    todo!()
    // let mut result = Vec::new();
    // result.push(input[0]);
    //
    //
    // let mut first = 0;
    // let mut middle = 1;
    // let mut last = 2;
    // let mut temp_area = 0f64;
    // while last < input.len() {
    //     // Calculate the area of the current triad
    //     // of points
    //     let area = triangle_area(input[first],
    //                              input[middle],
    //                              input[last]);
    //     // Check If it should retain points or not
    //     temp_area += area;
    //     if temp_area > area_threshold {
    //         temp_area = 0.0;
    //         result.push(input[middle]);
    //         //result.push(input[last]);
    //         first = middle;
    //         middle = first + 1;
    //         last = first + 2;
    //     }
    //     else {
    //         middle += 1;
    //         last += 1;
    //     }
    // }
    // result
    //
    // todo!()
}
