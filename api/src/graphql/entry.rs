//! The Entry object, expressed for GraphQL.

use crate::ManagedOplDb;
use opltypes::WeightUnits::Kg;
use opltypes::{Age, PointsSystem, WeightClassKg};

use crate::graphql::gqltypes;
use crate::graphql::{Lifter, Meet};

/// A unique entry in the database.
///
/// Each entry corresponds to a division placing.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Entry(pub u32);

#[graphql_object(context = ManagedOplDb)]
impl Entry {
    /// The meet in which the entry occurred.
    fn meet(&self, db: &ManagedOplDb) -> Meet {
        Meet(db.0.get_entry(self.0).meet_id)
    }

    /// The lifter corresponding to this entry.
    fn lifter(&self, db: &ManagedOplDb) -> Lifter {
        Lifter(db.0.get_entry(self.0).lifter_id)
    }

    /// The lifter's sex for this entry.
    fn sex(&self, db: &ManagedOplDb) -> String {
        format!("{}", db.0.get_entry(self.0).sex)
    }

    /// The event for this entry, like "SBD".
    fn event(&self, db: &ManagedOplDb) -> String {
        format!("{}", db.0.get_entry(self.0).event)
    }

    /// The equipment for this entry.
    fn equipment(&self, db: &ManagedOplDb) -> gqltypes::Equipment {
        db.0.get_entry(self.0).equipment.into()
    }

    /// The lifter's age at this entry.
    fn age(&self, db: &ManagedOplDb) -> Option<f64> {
        match db.0.get_entry(self.0).age {
            Age::None => None,
            age => Some(age.into()),
        }
    }

    /// The division for this entry.
    fn division(&self, db: &ManagedOplDb) -> Option<&str> {
        db.0.get_entry(self.0).get_division()
    }

    /// The lifter's bodyweight in kilograms.
    fn bodyweight_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).bodyweightkg.into()
    }
    /// The lifter's bodyweight in pounds.
    fn bodyweight_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).bodyweightkg.as_lbs().into()
    }

    /// The lifter's weightclass in kilograms.
    ///
    /// This is a String because SHW classes have a "+" suffix.
    fn weight_class_kg(&self, db: &ManagedOplDb) -> Option<String> {
        match db.0.get_entry(self.0).weightclasskg {
            WeightClassKg::None => None,
            wc => Some(format!("{}", wc)),
        }
    }
    /// The lifter's weightclass in pounds.
    ///
    /// This is a String because SHW classes have a "+" suffix.
    fn weight_class_lbs(&self, db: &ManagedOplDb) -> Option<String> {
        match db.0.get_entry(self.0).weightclasskg {
            WeightClassKg::None => None,
            wc => Some(format!("{}", wc.as_lbs())),
        }
    }

    /// The first squat attempt in kilograms.
    fn squat1_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).squat1kg.into()
    }
    /// The first squat attempt in pounds.
    fn squat1_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).squat1kg.as_lbs().into()
    }

    /// The second squat attempt in kilograms.
    fn squat2_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).squat2kg.into()
    }
    /// The second squat attempt in pounds.
    fn squat2_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).squat2kg.as_lbs().into()
    }

    /// The third squat attempt in kilograms.
    fn squat3_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).squat3kg.into()
    }
    /// The third squat attempt in pounds.
    fn squat3_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).squat3kg.as_lbs().into()
    }

    /// The fourth squat attempt in kilograms.
    fn squat4_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).squat4kg.into()
    }
    /// The third squat attempt in pounds.
    fn squat4_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).squat4kg.as_lbs().into()
    }

    /// The best squat of the first 3 attempts in kilograms.
    fn best3_squat_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).best3squatkg.into()
    }
    /// The best squat of the first 3 attempts in pounds.
    fn best3_squat_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).best3squatkg.as_lbs().into()
    }

    /// The best squat of the first 4 attempts in kilograms.
    fn best4_squat_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).highest_squatkg().into()
    }
    /// The best squat of the first 4 attempts in pounds.
    fn best4_squat_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).highest_squatkg().as_lbs().into()
    }

    /// The first bench attempt in kilograms.
    fn bench1_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).bench1kg.into()
    }
    /// The first bench attempt in pounds.
    fn bench1_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).bench1kg.as_lbs().into()
    }

    /// The second bench attempt in kilograms.
    fn bench2_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).bench2kg.into()
    }
    /// The second bench attempt in pounds.
    fn bench2_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).bench2kg.as_lbs().into()
    }

    /// The third bench attempt in kilograms.
    fn bench3_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).bench3kg.into()
    }
    /// The third bench attempt in pounds.
    fn bench3_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).bench3kg.as_lbs().into()
    }

    /// The fourth bench attempt in kilograms.
    fn bench4_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).bench4kg.into()
    }
    /// The third bench attempt in pounds.
    fn bench4_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).bench4kg.as_lbs().into()
    }

    /// The best bench of the first 3 attempts in kilograms.
    fn best3_bench_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).best3benchkg.into()
    }
    /// The best bench of the first 3 attempts in pounds.
    fn best3_bench_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).best3benchkg.as_lbs().into()
    }

    /// The best bench of the first 4 attempts in kilograms.
    fn best4_bench_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).highest_benchkg().into()
    }
    /// The best bench of the first 4 attempts in pounds.
    fn best4_bench_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).highest_benchkg().as_lbs().into()
    }

    /// The first deadlift attempt in kilograms.
    fn deadlift1_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).deadlift1kg.into()
    }
    /// The first deadlift attempt in pounds.
    fn deadlift1_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).deadlift1kg.as_lbs().into()
    }

    /// The second deadlift attempt in kilograms.
    fn deadlift2_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).deadlift2kg.into()
    }
    /// The second deadlift attempt in pounds.
    fn deadlift2_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).deadlift2kg.as_lbs().into()
    }

    /// The third deadlift attempt in kilograms.
    fn deadlift3_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).deadlift3kg.into()
    }
    /// The third deadlift attempt in pounds.
    fn deadlift3_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).deadlift3kg.as_lbs().into()
    }

    /// The fourth deadlift attempt in kilograms.
    fn deadlift4_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).deadlift4kg.into()
    }
    /// The fourth deadlift attempt in pounds.
    fn deadlift4_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).deadlift4kg.as_lbs().into()
    }

    /// The best deadlift of the first 3 attempts in kilograms.
    fn best3_deadlift_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).best3deadliftkg.into()
    }
    /// The best deadlift of the first 3 attempts in pounds.
    fn best3_deadlift_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).best3deadliftkg.as_lbs().into()
    }

    /// The best deadlift of the first 4 attempts in kilograms.
    fn best4_deadlift_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).highest_deadliftkg().into()
    }
    /// The best deadlift of the first 4 attempts in pounds.
    fn best4_deadlift_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).highest_deadliftkg().as_lbs().into()
    }

    /// The event total in kilograms.
    fn total_kg(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).totalkg.into()
    }
    /// The event total in pounds.
    fn total_lbs(&self, db: &ManagedOplDb) -> Option<f64> {
        db.0.get_entry(self.0).totalkg.as_lbs().into()
    }

    /// The entry's place.
    fn place(&self, db: &ManagedOplDb) -> String {
        format!("{}", db.0.get_entry(self.0).place)
    }

    /// AH points.
    fn ah(&self, db: &ManagedOplDb) -> f64 {
        db.0.get_entry(self.0).points(PointsSystem::AH, Kg).into()
    }
    /// Dots points.
    fn dots(&self, db: &ManagedOplDb) -> f64 {
        db.0.get_entry(self.0).points(PointsSystem::Dots, Kg).into()
    }
    /// Glossbrenner points.
    fn glossbrenner(&self, db: &ManagedOplDb) -> f64 {
        db.0.get_entry(self.0)
            .points(PointsSystem::Glossbrenner, Kg)
            .into()
    }
    /// IPF Goodlift points.
    fn goodlift(&self, db: &ManagedOplDb) -> f64 {
        db.0.get_entry(self.0)
            .points(PointsSystem::Goodlift, Kg)
            .into()
    }
    /// IPF points.
    fn ipf_points(&self, db: &ManagedOplDb) -> f64 {
        db.0.get_entry(self.0)
            .points(PointsSystem::IPFPoints, Kg)
            .into()
    }
    /// McCulloch points.
    fn mcculloch(&self, db: &ManagedOplDb) -> f64 {
        db.0.get_entry(self.0)
            .points(PointsSystem::McCulloch, Kg)
            .into()
    }
    /// NASA points.
    fn nasa(&self, db: &ManagedOplDb) -> f64 {
        db.0.get_entry(self.0).points(PointsSystem::NASA, Kg).into()
    }
    /// Reshel points.
    fn reshel(&self, db: &ManagedOplDb) -> f64 {
        db.0.get_entry(self.0)
            .points(PointsSystem::Reshel, Kg)
            .into()
    }
    /// Schwartz/Malone points.
    fn schwartz_malone(&self, db: &ManagedOplDb) -> f64 {
        db.0.get_entry(self.0)
            .points(PointsSystem::SchwartzMalone, Kg)
            .into()
    }
    /// Wilks points.
    fn wilks(&self, db: &ManagedOplDb) -> f64 {
        db.0.get_entry(self.0)
            .points(PointsSystem::Wilks, Kg)
            .into()
    }
    /// Wilks2020 points.
    fn wilks2020(&self, db: &ManagedOplDb) -> f64 {
        db.0.get_entry(self.0)
            .points(PointsSystem::Wilks2020, Kg)
            .into()
    }

    /// Whether this entry counts as drug-tested.
    fn tested(&self, db: &ManagedOplDb) -> bool {
        db.0.get_entry(self.0).tested
    }

    // TODO: AgeClass
    // TODO: BirthYearClass
    // TODO: LifterCountry
    // TODO: LifterState
}
