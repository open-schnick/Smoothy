use crate::failing_assertion;
use smoothy::{assert_that, PathAssertion};
use std::path::{Path, PathBuf};
use tempfile::{NamedTempFile, TempDir};

#[cfg(test)]
mod succeeds {
    use super::*;

    #[test]
    fn with_str() {
        let path_str = "/path/that/does/not/exist/file.txt";

        assert_that(path_str).not_exists();
    }

    #[test]
    fn with_string() {
        let path_string = String::from("/path/that/does/not/exist/file.txt");

        assert_that(path_string).not_exists();
    }

    #[test]
    fn with_path() {
        let path = Path::new("/path/that/does/not/exist/file.txt");

        assert_that(path).not_exists();
    }

    #[test]
    fn with_path_buf() {
        let path_buf = PathBuf::from("/path/that/does/not/exist/file.txt");

        assert_that(path_buf).not_exists();
    }
}

#[cfg(test)]
mod fails {
    use super::*;

    #[test]
    fn with_existing_file() {
        let file = NamedTempFile::new().unwrap();

        failing_assertion!(
            {
                assert_that(file.path()).not_exists();
            },
            "to point at an non existing entity in the filesystem"
        );
    }

    #[test]
    fn with_existing_directory() {
        let dir = TempDir::new().unwrap();

        failing_assertion!(
            {
                assert_that(dir.path()).not_exists();
            },
            "to point at an non existing entity in the filesystem"
        );
    }
}

#[cfg(test)]
#[cfg(unix)]
mod symlinks {
    use super::*;
    use std::os::unix::fs::symlink;

    #[test]
    fn pointing_to_file_fails() {
        let temp_dir = TempDir::new().unwrap();
        let target_file = NamedTempFile::new().unwrap();
        let symlink_path = temp_dir.path().join("symlink");

        symlink(target_file.path(), &symlink_path).unwrap();

        failing_assertion!(
            {
                assert_that(symlink_path).not_exists();
            },
            "to point at an non existing entity in the filesystem"
        );
    }

    #[test]
    fn pointing_to_directory_fails() {
        let temp_dir = TempDir::new().unwrap();
        let symlink_path = temp_dir.path().join("symlink");

        symlink(temp_dir.path(), &symlink_path).unwrap();

        failing_assertion!(
            {
                assert_that(symlink_path).not_exists();
            },
            "to point at an non existing entity in the filesystem"
        );
    }

    #[test]
    fn succeeds_when_symlink_is_broken() {
        let temp_dir = TempDir::new().unwrap();
        let non_existent_target = temp_dir.path().join("non_existent_target");
        let symlink_path = temp_dir.path().join("symlink");

        symlink(non_existent_target, &symlink_path).unwrap();

        assert_that(symlink_path).not_exists();
    }
}
