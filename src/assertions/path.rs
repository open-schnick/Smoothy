use crate::{implementation, private, Asserter};
use std::{
    fs::{self, File},
    path::Path,
};

/// Specifies various assertions on file system paths. Implemented on [`Asserter`]
///
/// This trait is sealed and cannot be implemented outside Smoothy.
pub trait PathAssertion<PathLike>: private::Sealed
where
    PathLike: AsRef<Path>,
{
    /// Asserts that the path exists in the file system
    ///
    /// It does not matter if the path points to an existing file or directory.
    /// For more see [`fs::exists`].
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// # use tempfile::NamedTempFile;
    /// let temp_file = NamedTempFile::new().unwrap();
    ///
    /// assert_that(temp_file.path()).exists();
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that("/path/that/does/not/exist").exists();
    /// ```
    ///
    /// # Panics
    /// When the path does not exist in the file system
    #[track_caller]
    fn exists(self) -> Asserter<File>;

    /// Asserts that the path does not exist in the file system
    ///
    /// For more see [`fs::exists`].
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// #
    /// assert_that("/path/that/does/not/exist").not_exists();
    /// ```
    ///
    /// ```should_panic
    /// # use smoothy::prelude::*;
    /// # use tempfile::NamedTempFile;
    /// let temp_file = NamedTempFile::new().unwrap();
    ///
    /// assert_that(temp_file.path()).not_exists();
    /// ```
    ///
    /// # Panics
    /// When the path exists in the file system
    #[track_caller]
    fn not_exists(self);

    /// Asserts that the path is a symlink
    ///
    /// This checks the metadata of the path without following symlinks to determine
    /// if it represents a symbolic link. Use this to verify that a path is actually
    /// a symlink, not just what it points to.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// # use std::os::unix::fs::symlink;
    /// # use std::fs::File;
    /// # use tempfile::TempDir;
    /// # let temp_dir = TempDir::new().unwrap();
    /// # let target = temp_dir.path().join("target");
    /// # File::create(&target).unwrap();
    /// # let link = temp_dir.path().join("link");
    /// symlink(&target, &link).unwrap();
    ///
    /// assert_that(&link).is_symlink();
    /// ```
    ///
    /// # Panics
    /// When the path is not a symlink or does not exist
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_symlink(self) -> Asserter<File>;

    /// Asserts that the path points to a regular file
    ///
    /// This checks the metadata of the path without following symlinks. If you want
    /// to check if a symlink's target is a file, use [`exists`](PathAssertion::exists)
    /// followed by [`is_file`](crate::FileAssertion::is_file).
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// # use tempfile::NamedTempFile;
    /// let temp_file = NamedTempFile::new().unwrap();
    ///
    /// assert_that(temp_file.path()).is_file();
    /// ```
    ///
    /// # Panics
    /// When the path does not point to a regular file or does not exist
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_file(self) -> Asserter<File>;

    /// Asserts that the path points to a directory
    ///
    /// This checks the metadata of the path without following symlinks. If you want
    /// to check if a symlink's target is a directory, use [`exists`](PathAssertion::exists)
    /// followed by [`is_directory`](crate::FileAssertion::is_directory).
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// # use tempfile::TempDir;
    /// let temp_dir = TempDir::new().unwrap();
    ///
    /// assert_that(temp_dir.path()).is_directory();
    /// ```
    ///
    /// # Panics
    /// When the path does not point to a directory or does not exist
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_directory(self) -> Asserter<File>;
}

impl<PathLike> PathAssertion<PathLike> for Asserter<PathLike>
where
    PathLike: AsRef<Path>,
{
    #[allow(clippy::expect_used)]
    fn exists(self) -> Asserter<File> {
        let path = self.value.as_ref();

        implementation::assert_no_expected(
            fs::exists(path).expect("Failed to check if path exists"),
            path,
            "to point at an existing entity in the filesystem",
        );

        Asserter {
            value: File::open(path).expect("Failed to open file"),
        }
    }

    #[allow(clippy::expect_used)]
    fn not_exists(self) {
        let path = self.value.as_ref();

        implementation::assert_no_expected(
            !fs::exists(path).expect("Failed to check if path exists"),
            path,
            "to point at an non existing entity in the filesystem",
        );
    }

    #[allow(clippy::expect_used)]
    fn is_symlink(self) -> Asserter<File> {
        let path = self.value.as_ref();
        let metadata = fs::symlink_metadata(path);

        implementation::assert_no_expected(metadata.is_ok(), path, "to exist");

        #[allow(clippy::unwrap_used)]
        implementation::assert_no_expected(metadata.unwrap().is_symlink(), path, "to be a symlink");

        Asserter {
            value: File::open(path).expect("Failed to open file"),
        }
    }

    #[allow(clippy::expect_used)]
    fn is_file(self) -> Asserter<File> {
        let path = self.value.as_ref();
        let metadata = fs::symlink_metadata(path);

        implementation::assert_no_expected(metadata.is_ok(), path, "to exist");

        #[allow(clippy::unwrap_used)]
        implementation::assert_no_expected(
            metadata.unwrap().is_file(),
            path,
            "to be a regular file",
        );

        Asserter {
            value: File::open(path).expect("Failed to open file"),
        }
    }

    #[allow(clippy::expect_used)]
    fn is_directory(self) -> Asserter<File> {
        let path = self.value.as_ref();
        let metadata = fs::symlink_metadata(path);

        implementation::assert_no_expected(metadata.is_ok(), path, "to exist");

        #[allow(clippy::unwrap_used)]
        implementation::assert_no_expected(metadata.unwrap().is_dir(), path, "to be a directory");

        Asserter {
            value: File::open(path).expect("Failed to open file"),
        }
    }
}
