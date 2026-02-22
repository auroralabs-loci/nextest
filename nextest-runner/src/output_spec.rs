// Copyright (c) The nextest Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Specifies how test output is represented.
//!
//! The [`OutputSpec`] trait abstracts over two modes of output storage:
//!
//! - [`LiveSpec`]: output stored in memory during live execution, using
//!   [`ChildSingleOutput`].
//! - [`RecordingSpec`]: output stored in recordings,
//!   using [`ZipStoreOutput`].
//!
//! Types generic over `S: OutputSpec` use `S::ChildOutput` for their output
//! fields. This enables adding additional associated types in the future
//! without changing every generic type's parameter list.

use crate::{record::ZipStoreOutput, test_output::ChildSingleOutput};

/// Specifies how test output is represented.
///
/// Two implementations exist:
///
/// - [`LiveSpec`]: output stored in memory during live execution.
/// - [`RecordingSpec`]: output stored in recordings.
pub trait OutputSpec {
    /// The type used to represent a single child output stream.
    type ChildOutput;
}

/// Output spec for live test execution.
///
/// Uses [`ChildSingleOutput`] for in-memory byte buffers with lazy UTF-8 string
/// conversion.
pub struct LiveSpec;

impl OutputSpec for LiveSpec {
    type ChildOutput = ChildSingleOutput;
}

/// Output spec for recorded/replayed test runs.
///
/// Uses [`ZipStoreOutput`] for content-addressed file references in zip
/// archives.
pub struct RecordingSpec;

impl OutputSpec for RecordingSpec {
    type ChildOutput = ZipStoreOutput;
}
