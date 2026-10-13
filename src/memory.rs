// SPDX-License-Identifier: MPL-2.0

use std::fs::File;
use std::io::{self, Read};

const KIB_PER_MB: u64 = 1024;
const KIB_PER_GB: u64 = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Memory {
    used_kib: u64,
    total_kib: u64,
}

impl Memory {
    pub fn used_gb(self) -> u64 {
        self.used_kib / KIB_PER_GB + u64::from(self.used_kib % KIB_PER_GB >= KIB_PER_GB / 2)
    }

    pub fn used_mb(self) -> u64 {
        self.used_kib / KIB_PER_MB
    }

    pub fn total_mb(self) -> u64 {
        self.total_kib / KIB_PER_MB
    }

    pub fn low_memory(self) -> bool {
        self.total_kib - self.used_kib <= self.total_kib / 10
    }
}

pub fn read_memory(buffer: &mut String) -> io::Result<Memory> {
    buffer.clear();
    File::open("/proc/meminfo")?.read_to_string(buffer)?;
    parse_memory(buffer)
}

fn parse_memory(meminfo: &str) -> io::Result<Memory> {
    let invalid = || {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid MemTotal or MemAvailable",
        )
    };
    let mut total = None;
    let mut available = None;
    for line in meminfo.lines() {
        let mut fields = line.split_ascii_whitespace();
        let target = match fields.next() {
            Some("MemTotal:") => &mut total,
            Some("MemAvailable:") => &mut available,
            _ => continue,
        };
        *target = Some(
            fields
                .next()
                .and_then(|value| value.parse::<u64>().ok())
                .ok_or_else(invalid)?,
        );
        if fields.next() != Some("kB") {
            return Err(invalid());
        }
        if total.is_some() && available.is_some() {
            break;
        }
    }
    let total = total.filter(|value| *value > 0).ok_or_else(invalid)?;
    let available = available.ok_or_else(invalid)?;
    // Linux reports KiB as "kB". MemAvailable includes reclaimable cache;
    // subtracting it matches modern free(1) and excludes swap.
    let used_kib = total.checked_sub(available).ok_or_else(invalid)?;
    total.checked_mul(1024).ok_or_else(invalid)?;
    Ok(Memory {
        used_kib,
        total_kib: total,
    })
}

#[cfg(test)]
mod tests {
    use super::{KIB_PER_GB, parse_memory};

    #[test]
    fn uses_available_not_free_and_ignores_swap() {
        let meminfo = "MemTotal: 67108864 kB\nMemFree: 1000000 kB\nMemAvailable: 19922944 kB\nSwapTotal: 9000000 kB\nSwapFree: 0 kB\n";
        assert_eq!(parse_memory(meminfo).unwrap().used_gb(), 45);
    }

    #[test]
    fn rounds_to_nearest_binary_gb() {
        for (used, expected) in [
            (0, 0),
            (KIB_PER_GB / 2 - 1, 0),
            (KIB_PER_GB / 2, 1),
            (KIB_PER_GB, 1),
        ] {
            let available = KIB_PER_GB - used;
            let meminfo = format!("MemAvailable: {available} kB\nMemTotal: {KIB_PER_GB} kB\n");
            assert_eq!(
                parse_memory(&meminfo).unwrap().used_gb(),
                expected,
                "{meminfo}"
            );
        }
    }

    #[test]
    fn full_usage_reflects_actual_capacity() {
        let memory = parse_memory("MemTotal: 64960512 kB\nMemAvailable: 0 kB\n").unwrap();
        assert_eq!(
            (memory.used_gb(), memory.used_mb(), memory.total_mb()),
            (62, 63438, 63438)
        );
    }

    #[test]
    fn low_memory_scales_with_total_and_includes_ten_percent() {
        for total in [1000, 8 * KIB_PER_GB, 128 * KIB_PER_GB] {
            for (available, low) in [
                (0, true),
                (total / 10 - 1, true),
                (total / 10, true),
                (total / 10 + 1, false),
                (total, false),
            ] {
                let meminfo = format!("MemTotal: {total} kB\nMemAvailable: {available} kB\n");
                assert_eq!(
                    parse_memory(&meminfo).unwrap().low_memory(),
                    low,
                    "{meminfo}"
                );
            }
        }
    }

    #[test]
    fn tooltip_measurements_use_whole_mb_without_losing_actual_total() {
        let memory = parse_memory("MemTotal: 64960513 kB\nMemAvailable: 4194304 kB\n").unwrap();
        assert_eq!((memory.used_mb(), memory.total_mb()), (59342, 63438));
    }

    #[test]
    fn rejects_missing_malformed_and_inconsistent_readings() {
        for meminfo in [
            "MemTotal: 100 kB\nMemFree: 20 kB\n",
            "MemTotal: 100 kB\nMemAvailable: 101 kB\n",
            "MemTotal: 0 kB\nMemAvailable: 0 kB\n",
            "MemTotal: nope kB\nMemAvailable: 20 kB\n",
            "MemTotal: 100 MB\nMemAvailable: 20 kB\n",
            "MemTotal: 18446744073709551615 kB\nMemAvailable: 0 kB\n",
        ] {
            assert_eq!(
                parse_memory(meminfo).unwrap_err().kind(),
                std::io::ErrorKind::InvalidData,
                "{meminfo}"
            );
        }
    }
}
