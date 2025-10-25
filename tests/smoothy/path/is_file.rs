use crate::failing_assertion;
use smoothy::{assert_that, AssertionConnector, PathAssertion};
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use tempfile::{NamedTempFile, TempDir};

#[cfg(test)]
mod succeeds {
    use super::*;

    #[test]
    fn with_regular_file() {
        let temp_file = NamedTempFile::new().unwrap();

        assert_that(temp_file.path()).is_file();
    }

    #[test]
    fn with_str() {
        let temp_file = NamedTempFile::new().unwrap();
        let path_str: &str = temp_file.path().to_str().unwrap();

        assert_that(path_str).is_file();
    }

    #[test]
    fn with_string() {
        let temp_file = NamedTempFile::new().unwrap();
        let path_string: String = temp_file.path().to_string_lossy().to_string();

        assert_that(path_string).is_file();
    }

    #[test]
    fn with_path() {
        let temp_file = NamedTempFile::new().unwrap();
        let path: &Path = temp_file.path();

        assert_that(path).is_file();
    }

    #[test]
    fn with_path_buf() {
        let temp_file = NamedTempFile::new().unwrap();
        let path_buf: PathBuf = temp_file.path().to_path_buf();

        assert_that(path_buf).is_file();
    }

    #[test]
    fn returns_assertion_connector() {
        let temp_file = NamedTempFile::new().unwrap();

        let _connector: AssertionConnector<File> = assert_that(temp_file.path()).is_file();
    }
}

#[cfg(test)]
mod fails {
    use super::*;

    #[test]
    fn with_directory() {
        let temp_dir = TempDir::new().unwrap();

        failing_assertion!(
            {
                assert_that(temp_dir.path()).is_file();
            },
            "to be a regular file"
        );
    }

    #[test]
    fn with_non_existent_file() {
        failing_assertion!(
            {
                assert_that("/path/that/does/not/exist").is_file();
            },
            "to exist"
        );
    }
}

#[cfg(test)]
#[cfg(unix)]
mod symlinks {
    use super::*;
    use std::os::unix::fs::symlink;

    #[test]
    fn fails_when_checking_symlink_itself() {
        let temp_dir = TempDir::new().unwrap();
        let target_file = NamedTempFile::new().unwrap();
        let link_path = temp_dir.path().join("link_to_file");

        symlink(target_file.path(), &link_path).unwrap();

        failing_assertion!(
            {
                assert_that(&link_path).is_file();
            },
            "to be a regular file"
        );
    }
}
