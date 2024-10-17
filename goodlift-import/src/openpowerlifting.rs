use serde::Serialize;

use crate::types::{Attempt, Placing};

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Row {
    pub name: String,
    pub country: String,
    pub sex: String,
    pub birth_date: Option<String>,
    pub birth_year: String,
    pub age: u8,
    pub division: String,
    pub weight_class_kg: String,
    pub bodyweight_kg: f32,
    pub squat_1_kg: Attempt,
    pub squat_2_kg: Attempt,
    pub squat_3_kg: Attempt,
    pub best_3_squat_kg: Attempt,
    pub bench_1_kg: Attempt,
    pub bench_2_kg: Attempt,
    pub bench_3_kg: Attempt,
    pub best_3_bench_kg: Attempt,
    pub deadlift_1_kg: Attempt,
    pub deadlift_2_kg: Attempt,
    pub deadlift_3_kg: Attempt,
    pub best_3_deadlift_kg: Attempt,
    pub total_kg: Attempt,
    pub place: Placing,
    pub event: String,
    pub equipment: String,
}
