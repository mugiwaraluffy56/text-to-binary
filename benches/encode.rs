use binrs::checksum::compute;
use binrs::encode::{encode, encode_base64, encode_gray, encode_run_length_binary, EncodeOpts};
use binrs::format::Format;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench_encode_bin(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_bin");
    for size in [64, 512, 4096, 65536] {
        let input: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            let opts = EncodeOpts {
                format: Format::Bin,
                sep: " ".to_string(),
                width: None,
                lsb_first: false,
                nibble_groups: false,
            };
            b.iter(|| encode(black_box(data), &opts));
        });
    }
    group.finish();
}

fn bench_encode_hex(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_hex");
    for size in [64, 4096, 65536] {
        let input: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            let opts = EncodeOpts {
                format: Format::Hex,
                sep: " ".to_string(),
                width: None,
                lsb_first: false,
                nibble_groups: false,
            };
            b.iter(|| encode(black_box(data), &opts));
        });
    }
    group.finish();
}

fn bench_encode_gray(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_gray");
    for size in [64, 4096, 65536] {
        let input: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            b.iter(|| encode_gray(black_box(data), " ", None));
        });
    }
    group.finish();
}

fn bench_base64(c: &mut Criterion) {
    let mut group = c.benchmark_group("base64_encode");
    for size in [64, 4096, 65536] {
        let input: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            b.iter(|| encode_base64(black_box(data)));
        });
    }
    group.finish();
}

fn bench_rle(c: &mut Criterion) {
    let mut group = c.benchmark_group("rle_encode");
    for size in [64, 4096] {
        let input: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            b.iter(|| encode_run_length_binary(black_box(data)));
        });
    }
    group.finish();
}

fn bench_crc32(c: &mut Criterion) {
    let mut group = c.benchmark_group("crc32");
    for size in [64, 4096, 65536] {
        let input: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, data| {
            b.iter(|| compute(black_box(data), "crc32"));
        });
    }
    group.finish();
}

fn bench_crc_all(c: &mut Criterion) {
    let input: Vec<u8> = (0..4096).map(|i| (i % 256) as u8).collect();
    c.bench_function("crc_all_4k", |b| {
        b.iter(|| compute(black_box(&input), "all"));
    });
}

criterion_group!(
    benches,
    bench_encode_bin,
    bench_encode_hex,
    bench_encode_gray,
    bench_base64,
    bench_rle,
    bench_crc32,
    bench_crc_all,
);
criterion_main!(benches);
