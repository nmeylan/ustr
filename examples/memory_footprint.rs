//! Interns many short identifiers plus the raft data set and prints
//! the cache footprint. Measure peak RSS with:
//! `/usr/bin/time -v cargo run --release --example memory_footprint`
use ustr::ustr;

const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

// Deterministic 2..=12 character identifier for index `j`.
fn short_identifier(j: u64, buf: &mut String) {
    let mut h = j.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0xD1B5_4A32_D192_ED03;
    buf.clear();
    for _ in 0..2 + h % 11 {
        h = h
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        buf.push(ALPHABET[(h >> 58) as usize % ALPHABET.len()] as char);
    }
}

fn main() {
    let mut buf = String::new();
    for i in 0..1_000_000u64 {
        // About 30% of the strings repeat an earlier one.
        let j = if i > 0 && i % 10 < 3 {
            i.wrapping_mul(0x9E37_79B9_7F4A_7C15) % i
        } else {
            i
        };
        short_identifier(j, &mut buf);
        ustr(&buf);
    }
    for s in include_str!("../data/raft-large-directories.txt").split_whitespace() {
        ustr(s);
    }
    println!("num_entries: {}", ustr::num_entries());
    println!("total_allocated: {}", ustr::total_allocated());
}
