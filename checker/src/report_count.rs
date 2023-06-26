use std::ops::AddAssign;

/// Aggregates the issues found when checking meet data.
#[derive(Copy, Clone, Debug, Default)]
pub struct ReportCount {
    errors: usize,
    warnings: usize,
}

impl ReportCount {
    /// Creates a new `ReportCount` given the number of errors and warnings.
    pub fn new(errors: usize, warnings: usize) -> Self {
        Self { errors, warnings }
    }

    /// Checks whether there are any errors or warnings in this report.
    pub fn any(&self) -> bool {
        self.errors > 0 || self.warnings > 0
    }

    /// Returns the number of errors in this report.
    pub fn errors(&self) -> usize {
        self.errors
    }

    /// Returns the number of warnings in this report.
    pub fn warnings(&self) -> usize {
        self.warnings
    }
}

impl AddAssign<ReportCount> for ReportCount {
    /// Allows the usage of the `+=` operator to combine reports together.
    fn add_assign(&mut self, rhs: ReportCount) {
        self.errors += rhs.errors;
        self.warnings += rhs.warnings;
    }
}
