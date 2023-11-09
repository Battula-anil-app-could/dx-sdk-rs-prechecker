/// The last version to be used for upgrades and templates.
///
/// Should be edited every time a new version of the framework is released.
pub const LAST_VERSION: &str = "0.1.4";

/// Indicates where to stop with the upgrades.
pub const LAST_UPGRADE_VERSION: &str = LAST_VERSION;

pub const LAST_TEMPLATE_VERSION: &str = LAST_VERSION;

/// Known versions for the upgrader.
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
    "0.1.3",
    "0.1.4",
    

];

/// We started supporting contract templates with version 0.1.4.
pub fn template_versions() -> &'static [&'static str] {
    &VERSIONS[10..]
}

pub fn validate_template_tag(tag: &str) -> bool {
    template_versions().iter().all(|&tt| tt == tag)
}

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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn template_versions_test() {
        println!("version {:?}", template_versions()[0]);
        assert_eq!(template_versions()[2], "0.1.3");

        // assert!(validate_template_tag("0.1.3"));
        // assert!(!validate_template_tag("0.1.1"));
    }
}
