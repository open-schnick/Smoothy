use crate::failing_assertion;
use smoothy::{assert_that, PathAssertion};
use std::path::{Path, PathBuf};
use tempfile::{NamedTempFile, TempDir};

#[cfg(test)]
mod succeeds {
    use super::*;
    use smoothy::FileAssertion;

    #[test]
    fn with_str() {
        let temp_file = NamedTempFile::new().unwrap();
        let path_str: &str = temp_file.path().to_str().unwrap();

        assert_that(path_str).exists();
    }

    #[test]
    fn with_string() {
        let temp_file = NamedTempFile::new().unwrap();
        let path_string: String = temp_file.path().to_string_lossy().to_string();

        assert_that(path_string).exists();
    }

    #[test]
    fn with_path() {
        let temp_file = NamedTempFile::new().unwrap();
        let path: &Path = temp_file.path();

        assert_that(path).exists();
    }

    #[test]
    fn with_path_buf() {
        let temp_file = NamedTempFile::new().unwrap();
        let path_buf: PathBuf = temp_file.path().to_path_buf();

        assert_that(path_buf).exists();
    }

    #[test]
    fn when_pointing_to_directory() {
        let temp_dir = TempDir::new().unwrap();
        let dir_path: &Path = temp_dir.path();

        assert_that(dir_path).exists();
    }

    #[test]
    fn and_allows_chaining() {
        let temp_file = NamedTempFile::new().unwrap();

        assert_that(temp_file.path()).exists().and().is_file();
    }
}

#[cfg(test)]
mod fails {
    use super::*;

    #[test]
    fn with_non_existent_path() {
        failing_assertion!({
            assert_that("/path/that/does/not/exist/file.txt").exists();
        });
    }
}

#[cfg(test)]
#[cfg(unix)]
mod symlinks {
    use super::*;
    use std::{fs::create_dir, os::unix::fs::symlink};

    #[test]
    fn pointing_to_file_succeeds() {
        let tempdir = TempDir::new().unwrap();
        let symlink_path = tempdir.path().join("symlink_to_file");
        let target_file = NamedTempFile::new().unwrap();

        symlink(target_file.path(), &symlink_path).unwrap();

        assert_that(&symlink_path).exists();
    }

    #[test]
    fn pointing_to_directory_succeeds() {
        let tempdir = TempDir::new().unwrap();
        let target_dir = tempdir.path().join("target_directory");
        create_dir(&target_dir).unwrap();
        let symlink_path = tempdir.path().join("symlink_to_directory");

        symlink(target_dir, &symlink_path).unwrap();

        assert_that(&symlink_path).exists();
    }

    #[test]
    fn fails_when_symlink_is_broken() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_which_does_not_exist = temp_dir.path().join("file-which-does-not-exist");
        let symlink_path = temp_dir.path().join("symlink");

        symlink(&file_which_does_not_exist, &symlink_path).unwrap();

        failing_assertion!(
            {
                assert_that(symlink_path).exists();
            },
            "to point at an existing entity in the filesystem"
        );
    }
}
