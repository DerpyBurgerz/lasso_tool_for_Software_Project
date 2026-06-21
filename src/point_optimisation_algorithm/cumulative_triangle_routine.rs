use bevy::math::Vec2;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use crate::triangle_area::triangle_area;

#[derive(Clone, Debug)]
pub struct CumulativeTriangleRoutine {
    pub anchor: usize,
    pub cumulative_area: f64,
    pub threshhold_area: f64,
}

impl Default for CumulativeTriangleRoutine {
    fn default() -> Self {
        CumulativeTriangleRoutine{
            anchor: 0,
            cumulative_area: 0.0,
            threshhold_area: 5.0,
        }
    }
}

pub fn cumulative_triangle_routine_step(points: &mut Vec<Vec2>, data: CumulativeTriangleRoutine) -> CumulativeTriangleRoutine {
    if points.len() <= 3 {
        return CumulativeTriangleRoutine::default();
    }

    let last_i = points.len()-1;
    let area = triangle_area(
        points[data.anchor],
        points[last_i-1],
        points[last_i]
    );

    // Check If it should retain points or not
    let temp_area = area + data.cumulative_area;
    if temp_area > data.threshhold_area {
        // leave the points in the list, they are fine
        // I don't think we have to do anything?
        CumulativeTriangleRoutine {
            anchor: last_i - 1,
            cumulative_area: 0.0,
            threshhold_area: data.threshhold_area,
        }
    } else {
        points[last_i - 2] = points[last_i - 1];
        points[last_i - 1] = points.pop().unwrap();
        CumulativeTriangleRoutine {
            cumulative_area: temp_area,
            ..data
        }
    }
}
