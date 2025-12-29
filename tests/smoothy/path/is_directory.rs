use crate::failing_assertion;
use smoothy::{assert_that, PathAssertion};
use std::path::{Path, PathBuf};
use tempfile::{NamedTempFile, TempDir};

#[cfg(test)]
mod succeeds {
    use super::*;
    use smoothy::FileAssertion;

    #[test]
    fn with_directory() {
        let temp_dir = TempDir::new().unwrap();

        assert_that(temp_dir.path()).is_directory();
    }

    #[test]
    fn with_str() {
        let temp_dir = TempDir::new().unwrap();
        let path_str: &str = temp_dir.path().to_str().unwrap();

        assert_that(path_str).is_directory();
    }

    #[test]
    fn with_string() {
        let temp_dir = TempDir::new().unwrap();
        let path_string: String = temp_dir.path().to_string_lossy().to_string();

        assert_that(path_string).is_directory();
    }

    #[test]
    fn with_path() {
        let temp_dir = TempDir::new().unwrap();
        let path: &Path = temp_dir.path();

        assert_that(path).is_directory();
    }

    #[test]
    fn with_path_buf() {
        let temp_dir = TempDir::new().unwrap();
        let path_buf: PathBuf = temp_dir.path().to_path_buf();

        assert_that(path_buf).is_directory();
    }

    #[test]
    fn and_allows_chaining() {
        let temp_dir = TempDir::new().unwrap();
        let path_buf: PathBuf = temp_dir.path().to_path_buf();

        assert_that(path_buf).is_directory().and().is_directory();
    }
}

#[cfg(test)]
mod fails {
    use super::*;

    #[test]
    fn with_regular_file() {
        let temp_file = NamedTempFile::new().unwrap();

        failing_assertion!(
            {
                assert_that(temp_file.path()).is_directory();
            },
            "to be a directory"
        );
    }

    #[test]
    fn with_non_existent_dir() {
        failing_assertion!(
            {
                assert_that("/path/that/does/not/exist").is_directory();
            },
            "to exist"
        );
    }
}

#[cfg(test)]
#[cfg(unix)]
mod symlinks {
    use super::*;
    use std::{fs::create_dir, os::unix::fs::symlink};

    #[test]
    fn fails_when_checking_symlink_itself() {
        let temp_dir = TempDir::new().unwrap();
        let target_dir = temp_dir.path().join("target");
        create_dir(&target_dir).unwrap();
        let link_path = temp_dir.path().join("link_to_dir");

        symlink(&target_dir, &link_path).unwrap();

        failing_assertion!(
            {
                assert_that(&link_path).is_directory();
            },
            "to be a directory"
        );
    }
}
