use crate::failing_assertion;
use smoothy::{assert_that, FileAssertion};
use std::fs::File;

#[cfg(test)]
mod succeeds {
    use super::*;
    use tempfile::tempfile;

    #[test]
    fn with_file() {
        let file = tempfile().unwrap();

        assert_that(file).is_file();
    }

    #[test]
    fn with_file_ref() {
        let file = tempfile().unwrap();

        assert_that(&file).is_file();
    }

    #[test]
    fn can_chain_assertions() {
        let file = tempfile().unwrap();

        assert_that(file).is_file().and().is_file().is_file();
    }
}

#[cfg(test)]
mod fails {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn with_directory() {
        let temp_dir = tempdir().unwrap();
        let dir = File::open(temp_dir.path()).unwrap();

        failing_assertion!(
            {
                assert_that(dir).is_file();
            },
            "to be a regular file"
        );
    }
}
