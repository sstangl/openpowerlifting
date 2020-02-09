//! Simple benchmarks to assess the speed of coefficient calculation.

#![feature(test)]

extern crate coefficients;
extern crate opltypes;
use opltypes::*;

mod benches {
    use super::*;

    extern crate test;
    use self::test::Bencher;

    // Constants used in the benchmarks below, to keep them all the same.
    const BODYWEIGHT_MEN: WeightKg = WeightKg::from_i32(93);
    const TOTAL_MEN: WeightKg = WeightKg::from_i32(93);

    const BODYWEIGHT_WOMEN: WeightKg = WeightKg::from_i32(63);
    const TOTAL_WOMEN: WeightKg = WeightKg::from_i32(475);

    /// Benchmarks AH.
    #[bench]
    fn ah(b: &mut Bencher) {
        b.iter(|| {
            coefficients::ah(Sex::M, BODYWEIGHT_MEN, TOTAL_MEN);
        })
    }

    /// Benchmarks Dots.
    #[bench]
    fn dots(b: &mut Bencher) {
        b.iter(|| {
            coefficients::wilks2020(Sex::M, BODYWEIGHT_MEN, TOTAL_MEN);
        })
    }

    /// Benchmarks Glossbrenner for men. Behaves differently than for women.
    #[bench]
    fn glossbrenner_men(b: &mut Bencher) {
        b.iter(|| {
            coefficients::glossbrenner(Sex::M, BODYWEIGHT_MEN, TOTAL_MEN);
        })
    }

    /// Benchmarks Glossbrenner for women. Behaves differently than for men.
    #[bench]
    fn glossbrenner_women(b: &mut Bencher) {
        b.iter(|| {
            coefficients::glossbrenner(Sex::F, BODYWEIGHT_WOMEN, TOTAL_WOMEN);
        })
    }

    /// Benchmarks Hoffman.
    #[bench]
    fn hoffman(b: &mut Bencher) {
        b.iter(|| {
            coefficients::hoffman(BODYWEIGHT_MEN, TOTAL_MEN);
        })
    }

    /// Benchmarks IPF points.
    #[bench]
    fn ipf(b: &mut Bencher) {
        b.iter(|| {
            coefficients::ipf(
                Sex::M,
                Equipment::Raw,
                Event::sbd(),
                BODYWEIGHT_MEN,
                TOTAL_MEN,
            );
        })
    }

    /// Benchmarks NASA.
    #[bench]
    fn nasa(b: &mut Bencher) {
        b.iter(|| {
            coefficients::hoffman(BODYWEIGHT_MEN, TOTAL_MEN);
        })
    }

    /// Benchmarks Reshel.
    #[bench]
    fn reshel(b: &mut Bencher) {
        b.iter(|| {
            coefficients::reshel(Sex::M, BODYWEIGHT_MEN, TOTAL_MEN);
        })
    }

    /// Benchmarks Schwartz/Malone for men.
    #[bench]
    fn schwartzmalone_men(b: &mut Bencher) {
        b.iter(|| {
            coefficients::schwartzmalone(Sex::M, BODYWEIGHT_MEN, TOTAL_MEN);
        })
    }

    /// Benchmarks Schwartz/Malone for women.
    #[bench]
    fn schwartzmalone_women(b: &mut Bencher) {
        b.iter(|| {
            coefficients::schwartzmalone(Sex::F, BODYWEIGHT_MEN, TOTAL_MEN);
        })
    }

    /// Benchmarks Wilks.
    #[bench]
    fn wilks(b: &mut Bencher) {
        b.iter(|| {
            coefficients::wilks(Sex::M, BODYWEIGHT_MEN, TOTAL_MEN);
        })
    }

    /// Benchmarks Wilks2020.
    #[bench]
    fn wilks2020(b: &mut Bencher) {
        b.iter(|| {
            coefficients::wilks2020(Sex::M, BODYWEIGHT_MEN, TOTAL_MEN);
        })
    }
}
