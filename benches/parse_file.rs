//! Benchmark ARINC 424 file parsing throughput and memory use.
//!
//! Uses the same FAA CIFP dump as `tests/test_file_parse.rs`.
//!
//! ```bash
//! cargo bench --bench parse_file --features rev18_faa
//!
//! # Optional JSON output path (defaults to target/bench_memory_report.json)
//! BENCH_MEMORY_JSON=target/parse_file_memory.json cargo bench --bench parse_file --features rev18_faa
//! ```

mod common;

use arinc424_rs::parser::{Arinc424Parser, Arinc424Version, Arinc424VersionedRecord};
use common::memory::{BenchMemoryDocument, BenchMemoryReport, MemoryReport, MemoryTracker};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::fs::File;
use std::hint::black_box;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::{Duration, Instant};

const DATA_FILE: &str = "data/FAACIFP18.txt";

struct ParseRunStats {
    lines_read: u64,
    records_parsed: u64,
    parse_errors: u64,
    elapsed: Duration,
}

fn load_lines(path: &Path) -> Vec<String> {
    let file = File::open(path).unwrap_or_else(|e| panic!("failed to open {}: {e}", path.display()));
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.expect("failed to read line"))
        .filter(|line| !line.is_empty())
        .collect()
}

fn parse_streaming(lines: &[String], track_memory: bool) -> (ParseRunStats, MemoryReport) {
    let parser = Arinc424Parser::new(Arinc424Version::Rev18FAA);
    let started = Instant::now();
    let mut memory = track_memory.then(MemoryTracker::new);
    let mut lines_read: u64 = 0;
    let mut records_parsed = 0;
    let mut parse_errors = 0;

    for line in lines {
        lines_read += 1;
        if let Some(tracker) = memory.as_mut() {
            tracker.observe(lines_read);
        }

        match parser.parse(line.as_bytes()) {
            Ok(record) => {
                black_box(record);
                records_parsed += 1;
            }
            Err(_) => parse_errors += 1,
        }
    }

    (
        ParseRunStats {
            lines_read,
            records_parsed,
            parse_errors,
            elapsed: started.elapsed(),
        },
        memory
            .map(|tracker| tracker.finish())
            .unwrap_or_else(MemoryReport::disabled),
    )
}

fn parse_collected(lines: &[String], track_memory: bool) -> (ParseRunStats, MemoryReport) {
    let parser = Arinc424Parser::new(Arinc424Version::Rev18FAA);
    let started = Instant::now();
    let mut memory = track_memory.then(MemoryTracker::new);
    let mut records: Vec<Arinc424VersionedRecord<'_>> = Vec::with_capacity(lines.len());
    let mut lines_read: u64 = 0;
    let mut records_parsed = 0;
    let mut parse_errors = 0;

    for line in lines {
        lines_read += 1;
        if let Some(tracker) = memory.as_mut() {
            tracker.observe(lines_read);
        }

        match parser.parse(line.as_bytes()) {
            Ok(record) => {
                records.push(record);
                records_parsed += 1;
            }
            Err(_) => parse_errors += 1,
        }
    }

    black_box(&records);

    (
        ParseRunStats {
            lines_read,
            records_parsed,
            parse_errors,
            elapsed: started.elapsed(),
        },
        memory
            .map(|tracker| tracker.finish())
            .unwrap_or_else(MemoryReport::disabled),
    )
}

fn to_memory_report(
    label: &str,
    stats: &ParseRunStats,
    memory: MemoryReport,
    file_bytes: u64,
) -> BenchMemoryReport {
    let records_per_sec = stats.records_parsed as f64 / stats.elapsed.as_secs_f64();

    BenchMemoryReport::new(label, stats.elapsed, memory)
        .with_metric("file_bytes", file_bytes)
        .with_metric("lines_read", stats.lines_read)
        .with_metric("records_parsed", stats.records_parsed)
        .with_metric("parse_errors", stats.parse_errors)
        .with_metric("records_per_sec", records_per_sec)
}

fn bench_parse_file(c: &mut Criterion) {
    let path = Path::new(DATA_FILE);
    if !path.exists() {
        eprintln!(
            "benchmark skipped: {DATA_FILE} not found (download FAA CIFP dump to data/)"
        );
        return;
    }

    let file_bytes = std::fs::metadata(path)
        .expect("failed to stat benchmark file")
        .len();
    let lines = load_lines(path);
    let non_empty_lines = lines.len() as u64;

    let mut group = c.benchmark_group("parse_file");
    group.throughput(Throughput::Elements(non_empty_lines));

    group.bench_function(BenchmarkId::new("rev18_faa", "streaming"), |b| {
        b.iter(|| {
            black_box(parse_streaming(&lines, false).0);
        });
    });

    group.bench_function(BenchmarkId::new("rev18_faa", "collected"), |b| {
        b.iter(|| {
            black_box(parse_collected(&lines, false).0);
        });
    });

    group.finish();

    let (streaming_stats, streaming_memory) = parse_streaming(&lines, true);
    let (collected_stats, collected_memory) = parse_collected(&lines, true);

    let document = BenchMemoryDocument::new(vec![
        to_memory_report("rev18_faa streaming", &streaming_stats, streaming_memory, file_bytes),
        to_memory_report("rev18_faa collected", &collected_stats, collected_memory, file_bytes),
    ]);

    document.print_stdout();
    match document.write_json_from_env() {
        Ok(path) => println!("\nmemory report written to {}", path.display()),
        Err(error) => eprintln!("\nfailed to write memory report json: {error}"),
    }
}

criterion_group!(benches, bench_parse_file);
criterion_main!(benches);
