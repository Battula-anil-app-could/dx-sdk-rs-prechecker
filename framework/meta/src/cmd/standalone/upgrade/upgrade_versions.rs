/// Not necessarily the last entry in `VERSIONS`.
///
/// Indicates where to stop with the upgrades.
pub const DEFAULT_LAST_VERSION: &str = "0.1.2";

/// Known version for the upgrader.
#[rustfmt::skip]
pub const VERSIONS: &[&str] = &[
    "0.0.1",
    "0.0.2",
    "0.0.3",
    "0.0.4",
    "0.0.5",
    "0.0.6",
    "0.0.7",
    "0.0.8",
    "0.0.9",
    "0.1.0",
    "0.1.1",
    "0.1.2",
];

pub struct VersionIterator {
    next_version: usize,
    last_version: String,
}

impl VersionIterator {
    fn is_last_version(&self, version: &str) -> bool {
        self.last_version == version
    }
}

impl Iterator for VersionIterator {
    type Item = (&'static str, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_version > 0 && self.next_version < VERSIONS.len() {
            let from_version = VERSIONS[self.next_version - 1];

            if self.is_last_version(from_version) {
                None
            } else {
                let to_version = VERSIONS[self.next_version];
                let result = (from_version, to_version);
                self.next_version += 1;
                Some(result)
            }
        } else {
            None
        }
    }
}

pub fn versions_iter(last_version: String) -> VersionIterator {
    VersionIterator {
        next_version: 1,
        last_version,
    }
}
