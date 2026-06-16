use bevy::math::Vec2;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
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
    if points.len() <= 2 {
        return data;
    }

    let area = triangle_area(points[data.first],
                             points[data.middle],
                             points[data.last]
    );

    // Check If it should retain points or not
    let mut temp_area = area + data.temp_area;

    if temp_area > data.threshhold_area {
        temp_area = 0.0;
        result.push(input[middle]);
        //result.push(input[last]);
        first = middle;
        middle = first + 1;
        last = first + 2;
    }
    else {
        middle += 1;
        last += 1;
    }

    todo!()
}
