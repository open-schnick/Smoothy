use crate::failing_assertion;
use smoothy::{assert_that, PathAssertion};
use std::path::Path;
use tempfile::{NamedTempFile, TempDir};

#[cfg(test)]
#[cfg(unix)]
mod succeeds {
    use super::*;
    use smoothy::FileAssertion;
    use std::os::unix::fs::symlink;

    #[test]
    fn with_symlink_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let target = NamedTempFile::new().unwrap();
        let link_path = temp_dir.path().join("link_to_file");

        symlink(target.path(), &link_path).unwrap();

        assert_that(&link_path).is_symlink();
    }

    #[test]
    fn with_symlink_to_directory() {
        let temp_dir = TempDir::new().unwrap();
        let target_dir = TempDir::new().unwrap();
        let link_path = temp_dir.path().join("link_to_dir");

        symlink(target_dir.path(), &link_path).unwrap();

        assert_that(&link_path).is_symlink();
    }

    #[test]
    fn with_str() {
        let temp_dir = TempDir::new().unwrap();
        let target = NamedTempFile::new().unwrap();
        let link_path = temp_dir.path().join("link");

        symlink(target.path(), &link_path).unwrap();

        let path_str: &str = link_path.to_str().unwrap();
        assert_that(path_str).is_symlink();
    }

    #[test]
    fn with_string() {
        let temp_dir = TempDir::new().unwrap();
        let target = NamedTempFile::new().unwrap();
        let link_path = temp_dir.path().join("link");

        symlink(target.path(), &link_path).unwrap();

        let path_string: String = link_path.to_string_lossy().to_string();
        assert_that(path_string).is_symlink();
    }

    #[test]
    fn with_path() {
        let temp_dir = TempDir::new().unwrap();
        let target = NamedTempFile::new().unwrap();
        let link_path = temp_dir.path().join("link");

        symlink(target.path(), &link_path).unwrap();

        let path: &Path = &link_path;
        assert_that(path).is_symlink();
    }

    #[test]
    fn with_path_buf() {
        let temp_dir = TempDir::new().unwrap();
        let target = NamedTempFile::new().unwrap();
        let link_path = temp_dir.path().join("link");

        symlink(target.path(), &link_path).unwrap();

        assert_that(link_path).is_symlink();
    }

    #[test]
    fn and_allows_chaining() {
        let temp_dir = TempDir::new().unwrap();
        let target = NamedTempFile::new().unwrap();
        let link_path = temp_dir.path().join("link");

        symlink(target.path(), &link_path).unwrap();

        assert_that(link_path).is_symlink().and().is_file();
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
                assert_that(temp_file.path()).is_symlink();
            },
            "to be a symlink"
        );
    }

    #[test]
    fn with_directory() {
        let temp_dir = TempDir::new().unwrap();

        failing_assertion!(
            {
                assert_that(temp_dir.path()).is_symlink();
            },
            "to be a symlink"
        );
    }

    #[test]
    fn with_non_existent_path() {
        failing_assertion!(
            {
                assert_that("/path/that/does/not/exist").is_symlink();
            },
            "to exist"
        );
    }
}
