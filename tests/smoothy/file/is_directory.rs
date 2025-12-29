use crate::failing_assertion;
use smoothy::{assert_that, FileAssertion};
use std::fs::File;
use tempfile::TempDir;

#[cfg(test)]
mod succeeds {
    use super::*;

    #[test]
    fn with_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir = File::open(temp_dir.path()).unwrap();

        assert_that(dir).is_directory();
    }

    #[test]
    fn with_directory_ref() {
        let temp_dir = TempDir::new().unwrap();
        let dir = File::open(temp_dir.path()).unwrap();

        assert_that(&dir).is_directory();
    }

    #[test]
    fn can_chain_assertions() {
        let temp_dir = TempDir::new().unwrap();
        let dir = File::open(temp_dir.path()).unwrap();

        assert_that(dir).is_directory().and().is_directory();
    }
}

#[cfg(test)]
mod fails {
    use super::*;
    use tempfile::tempfile;

    #[test]
    fn with_regular_file() {
        let file = tempfile().unwrap();

        failing_assertion!(
            {
                assert_that(file).is_directory();
            },
            "to be a directory"
        );
    }
}
