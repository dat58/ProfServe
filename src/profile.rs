use crate::error::{Error, Result};
use std::time::Duration;

/// CPU profiler utility.
// Inspired by https://github.com/datafuselabs/databend/blob/67f445e83cd4eceda98f6c1c114858929d564029/src/common/base/src/base/profiling.rs
#[derive(Debug)]
pub struct Profiling {
    duration: Duration,
    frequency: i32,
}

impl Profiling {
    /// Create a new CPU profiler.
    ///
    /// The profiler will collect CPU profiles at a given `frequency` for the
    /// given `duration`.
    ///
    pub fn new(duration: Duration, frequency: i32) -> Self {
        Self {
            duration,
            frequency,
        }
    }

    /// Generates a CPU profile report for the specified duration and frequency.
    ///
    /// This asynchronous function initializes a profiler guard with the given
    /// frequency and excludes specific libraries from profiling. It then waits
    /// for the specified duration before generating the CPU profile report.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `pprof::Report` on success, or an
    /// `Error` on failure.
    pub async fn report(&self) -> Result<pprof::Report> {
        let guard = pprof::ProfilerGuardBuilder::default()
            .frequency(self.frequency)
            .blocklist(&["libc", "libgcc", "pthread", "vdso"])
            .build()?;
        actix_web::rt::time::sleep(self.duration).await;
        guard.report().build().map_err(Error::from)
    }

    /// Dumps the CPU profile report as a formatted text string.
    ///
    /// This asynchronous function generates a CPU profile report for the
    /// specified duration and frequency, and then formats the report into
    /// a human-readable text string.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the formatted text string on success,
    /// or an `Error` on failure.
    pub async fn dump_text(&self) -> Result<String> {
        let report = self.report().await?;
        let text = format!("{report:?}");
        Ok(text)
    }

    /// Dumps the CPU profile report as a flamegraph.
    ///
    /// This asynchronous function generates a CPU profile report for the
    /// specified duration and frequency, and then converts the report into
    /// a flamegraph in binary format.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the flamegraph as a vector of bytes on
    /// success, or an `Error` on failure.
    pub async fn dump_flamegraph(&self) -> Result<Vec<u8>> {
        let report = self.report().await?;
        let mut body: Vec<u8> = Vec::new();
        report.flamegraph(&mut body)?;
        Ok(body)
    }
}
