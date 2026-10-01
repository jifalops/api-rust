use std::error::Error;

use tracing::error;

use crate::AppError;

/// Central seam for reporting errors. Currently logs through `tracing`; swap the
/// bodies for a real error-tracking backend (Sentry, Honeycomb, …) without
/// touching any call site.
pub struct ErrorReportingService {}

impl Default for ErrorReportingService {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorReportingService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn report_app_error(&self, err: &AppError) {
        error!("Reporting error: {}", err);
    }

    pub fn report_unknown_error(&self, err: &dyn Error) {
        error!("Reporting unknown error: {}", err);
    }
}
