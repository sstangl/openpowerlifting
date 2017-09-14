infer_schema!("../build/openpowerlifting.sqlite3");

#[derive(Queryable)]
pub struct Entry {
    pub id: Option<i32>,
    pub meetid: Option<i32>,
    pub name: Option<String>,
    pub sex: Option<String>,
    pub event: Option<String>,
    pub equipment: Option<String>,
    pub age: Option<f32>,
    pub division: Option<String>,
    pub bodyweightkg: Option<f32>,
    pub weightclasskg: Option<f32>,
    pub squat1kg: Option<f32>,
    pub squat2kg: Option<f32>,
    pub squat3kg: Option<f32>,
    pub squat4kg: Option<f32>,
    pub bestsquatkg: Option<f32>,
    pub bench1kg: Option<f32>,
    pub bench2kg: Option<f32>,
    pub bench3kg: Option<f32>,
    pub bench4kg: Option<f32>,
    pub bestbenchkg: Option<f32>,
    pub deadlift1kg: Option<f32>,
    pub deadlift2kg: Option<f32>,
    pub deadlift3kg: Option<f32>,
    pub deadlift4kg: Option<f32>,
    pub bestdeadliftkg: Option<f32>,
    pub totalkg: Option<f32>,
    pub place: Option<String>,
    pub wilks: Option<f32>,
    pub mcculloch: Option<f32>,
}

#[derive(Queryable)]
pub struct Meet {
    pub id: Option<i32>,
    pub path: Option<String>,
    pub federation: Option<String>,
    pub date: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub town: Option<String>,
    pub name: Option<String>,
}
