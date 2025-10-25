use crate::{implementation, private, AssertionConnector, BasicAsserter};
use std::{borrow::Borrow, fmt::Debug, fs::File};

/// Specifies various assertions on file handles. Implemented on [`BasicAsserter`]
///
/// This trait is sealed and cannot be implemented outside Smoothy.
pub trait FileAssertion<FileLike>: private::Sealed
where
    FileLike: Borrow<File> + Debug,
{
    /// Asserts that the file handle points to a regular file
    ///
    /// This checks the metadata of the file handle to determine if it represents
    /// a regular file (not a directory or symlink).
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// # use tempfile::NamedTempFile;
    /// # use std::fs::File;
    /// # let temp_file = NamedTempFile::new().unwrap();
    /// # let file = File::open(temp_file.path()).unwrap();
    /// assert_that(file).is_file();
    /// ```
    ///
    /// # Panics
    /// When the file handle does not point to a regular file
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_file(self) -> AssertionConnector<FileLike>;

    /// Asserts that the file handle points to a directory
    ///
    /// This checks the metadata of the file handle to determine if it represents
    /// a directory.
    ///
    /// # Examples
    /// ```
    /// # use smoothy::prelude::*;
    /// # use tempfile::TempDir;
    /// # use std::fs::File;
    /// # let temp_dir = TempDir::new().unwrap();
    /// # let dir = File::open(temp_dir.path()).unwrap();
    /// assert_that(dir).is_directory();
    /// ```
    ///
    /// # Panics
    /// When the file handle does not point to a directory
    #[track_caller]
    #[allow(clippy::wrong_self_convention)]
    fn is_directory(self) -> AssertionConnector<FileLike>;
}

impl<FileLike> FileAssertion<FileLike> for BasicAsserter<FileLike>
where
    FileLike: Borrow<File> + Debug,
{
    #[allow(clippy::expect_used)]
    fn is_file(self) -> AssertionConnector<FileLike> {
        let file: &File = self.value.borrow();
        let metadata = file.metadata().expect("Failed to read file metadata");

        implementation::assert_no_expected(metadata.is_file(), file, "to be a regular file");

        AssertionConnector { value: self.value }
    }

    #[allow(clippy::expect_used)]
    fn is_directory(self) -> AssertionConnector<FileLike> {
        let file: &File = self.value.borrow();
        let metadata = file.metadata().expect("Failed to read file metadata");

        implementation::assert_no_expected(metadata.is_dir(), file, "to be a directory");

        AssertionConnector { value: self.value }
    }
}
