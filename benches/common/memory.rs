//! Shared memory sampling and JSON reporting for benchmarks.
//!
//! # Example
//!
//! ```ignore
//! mod common;
//!
//! use common::memory::{BenchMemoryDocument, BenchMemoryReport, MemoryTracker};
//! use std::time::Instant;
//!
//! let started = Instant::now();
//! let mut memory = MemoryTracker::new();
//! for (i, _) in work_items.enumerate() {
//!     memory.observe(i as u64 + 1);
//!     // ...
//! }
//! let report = BenchMemoryReport::new("my_bench/streaming", started.elapsed(), memory.finish())
//!     .with_metric("items", item_count);
//!
//! BenchMemoryDocument::new(vec![report]).write_json_from_env()?;
//! ```

use serde::Serialize;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const SAMPLE_INTERVAL: u64 = 10_000;

pub const DEFAULT_JSON_PATH: &str = "target/bench_memory_report.json";
pub const JSON_OUTPUT_ENV: &str = "BENCH_MEMORY_JSON";

#[derive(Debug, Clone, Copy, Serialize)]
pub struct MemorySample {
    pub physical_kb: u64,
    pub virtual_kb: u64,
    pub physical_mib: u64,
    pub virtual_mib: u64,
}

impl MemorySample {
    fn from_stats(physical_mem: usize, virtual_mem: usize) -> Self {
        let physical_kb = (physical_mem / 1024) as u64;
        let virtual_kb = (virtual_mem / 1024) as u64;
        Self {
            physical_kb,
            virtual_kb,
            physical_mib: physical_kb / 1024,
            virtual_mib: virtual_kb / 1024,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MemoryReport {
    pub baseline: Option<MemorySample>,
    pub peak: Option<MemorySample>,
    pub final_memory: Option<MemorySample>,
    pub peak_delta_physical_kb: Option<u64>,
    pub peak_delta_virtual_kb: Option<u64>,
    pub peak_delta_physical_mib: Option<u64>,
    pub peak_delta_virtual_mib: Option<u64>,
}

impl MemoryReport {
    pub fn disabled() -> Self {
        Self::from_samples(None, None, None)
    }

    fn from_samples(
        baseline: Option<MemorySample>,
        peak: Option<MemorySample>,
        final_memory: Option<MemorySample>,
    ) -> Self {
        let peak_delta_physical_kb = match (baseline, peak) {
            (Some(base), Some(peak)) => Some(peak.physical_kb.saturating_sub(base.physical_kb)),
            _ => None,
        };
        let peak_delta_virtual_kb = match (baseline, peak) {
            (Some(base), Some(peak)) => Some(peak.virtual_kb.saturating_sub(base.virtual_kb)),
            _ => None,
        };

        Self {
            baseline,
            peak,
            final_memory,
            peak_delta_physical_mib: peak_delta_physical_kb.map(|kb| kb / 1024),
            peak_delta_virtual_mib: peak_delta_virtual_kb.map(|kb| kb / 1024),
            peak_delta_physical_kb,
            peak_delta_virtual_kb,
        }
    }
}

fn sample_current() -> Option<MemorySample> {
    memory_stats::memory_stats()
        .map(|stats| MemorySample::from_stats(stats.physical_mem, stats.virtual_mem))
}

fn peak_sample(
    current: Option<MemorySample>,
    sample: Option<MemorySample>,
) -> Option<MemorySample> {
    match (current, sample) {
        (Some(left), Some(right)) => Some(MemorySample {
            physical_kb: left.physical_kb.max(right.physical_kb),
            virtual_kb: left.virtual_kb.max(right.virtual_kb),
            physical_mib: left.physical_mib.max(right.physical_mib),
            virtual_mib: left.virtual_mib.max(right.virtual_mib),
        }),
        (None, Some(value)) | (Some(value), None) => Some(value),
        (None, None) => None,
    }
}

pub struct MemoryTracker {
    baseline: Option<MemorySample>,
    peak: Option<MemorySample>,
}

impl MemoryTracker {
    pub fn new() -> Self {
        let baseline = sample_current();
        Self {
            peak: baseline,
            baseline,
        }
    }

    pub fn observe(&mut self, iteration: u64) {
        if iteration.is_multiple_of(SAMPLE_INTERVAL) {
            self.peak = peak_sample(self.peak, sample_current());
        }
    }

    pub fn finish(mut self) -> MemoryReport {
        self.peak = peak_sample(self.peak, sample_current());
        MemoryReport::from_samples(self.baseline, self.peak, sample_current())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BenchMemoryReport {
    pub label: String,
    pub elapsed_secs: f64,
    pub memory: MemoryReport,
    #[serde(skip_serializing_if = "serde_json::Map::is_empty")]
    pub metrics: serde_json::Map<String, serde_json::Value>,
}

impl BenchMemoryReport {
    pub fn new(label: impl Into<String>, elapsed: Duration, memory: MemoryReport) -> Self {
        Self {
            label: label.into(),
            elapsed_secs: elapsed.as_secs_f64(),
            memory,
            metrics: serde_json::Map::new(),
        }
    }

    pub fn with_metric(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        if let Ok(value) = serde_json::to_value(value) {
            self.metrics.insert(key.into(), value);
        }
        self
    }

    fn print_stdout(&self) {
        println!("\n=== {} ===", self.label);
        println!(
            "elapsed: {:.3?}",
            Duration::from_secs_f64(self.elapsed_secs)
        );
        for (key, value) in &self.metrics {
            println!("{key}: {value}");
        }
        print_memory_line("baseline memory", self.memory.baseline);
        print_memory_line("peak memory", self.memory.peak);
        print_memory_line("final memory", self.memory.final_memory);
        if let Some(delta_mib) = self.memory.peak_delta_physical_mib {
            println!("peak delta (physical): {delta_mib} MiB");
        }
    }
}

fn print_memory_line(label: &str, sample: Option<MemorySample>) {
    match sample {
        Some(sample) => println!(
            "{label}: {} MiB physical / {} MiB virtual",
            sample.physical_mib, sample.virtual_mib
        ),
        None => println!("{label}: unavailable on this platform"),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BenchMemoryDocument {
    pub generated_at_unix_secs: u64,
    pub runs: Vec<BenchMemoryReport>,
}

impl BenchMemoryDocument {
    pub fn new(runs: Vec<BenchMemoryReport>) -> Self {
        let generated_at_unix_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        Self {
            generated_at_unix_secs,
            runs,
        }
    }

    pub fn write_json<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        if let Some(parent) = path.as_ref().parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        fs::write(path.as_ref(), json)
    }

    pub fn write_json_from_env(&self) -> io::Result<PathBuf> {
        let path = std::env::var(JSON_OUTPUT_ENV)
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(DEFAULT_JSON_PATH));
        self.write_json(&path)?;
        Ok(path)
    }

    pub fn print_stdout(&self) {
        for run in &self.runs {
            run.print_stdout();
        }
    }
}
