//! Builds wheels from a crate that exposes python bindings through pyo3
//!
//! The high-level api is [BuildOptions], which can be converted into the [BuildContext], which
//! then uses [compile()] and builds the appropriate wheels.
//!
//! # Cargo features
//!
//! - upload: Uses rewqest to add the upload command. By default this uses native openssl and
//! is therefore not manylinux comopliant
//!
//! - musl: Switches from native openssl to statically linked openssl, which makes the upload
//! feature manylinux compliant
//!
//! - password-storage (off by default): Uses the keyring package to store the password. keyring
//! pulls in a lot of shared libraries and outdated dependencies, so this is off by default, except
//! for the build on the github releases page.
//! (https://github.com/hwchen/secret-service-rs/issues/9)
//!
//! - auditwheel: Reimplements the more important part of the auditwheel
//! package in rust. Every  wheel is check unless [skip_auditwheel](BuildContext.skip_auditwheel) is
//! set to true.
//!
//! - human-panic (off by default): Adds human-panic, pulling in some outdated dependencies
//! (https://github.com/rust-clique/human-panic/pull/47)

#![deny(missing_docs)]

#[cfg(feature = "auditwheel")]
pub use crate::auditwheel::{auditwheel_rs, AuditWheelError};
pub use crate::build_context::BridgeModel;
pub use crate::build_context::BuildContext;
pub use crate::build_options::BuildOptions;
pub use crate::cargo_toml::CargoToml;
pub use crate::compile::compile;
pub use crate::develop::develop;
pub use crate::metadata::{Metadata21, WheelMetadata};
pub use crate::python_interpreter::PythonInterpreter;
pub use crate::target::Target;
#[cfg(feature = "upload")]
pub use {
    crate::registry::Registry,
    crate::upload::{upload, upload_wheels, UploadError},
};

#[cfg(feature = "auditwheel")]
mod auditwheel;
mod build_context;
mod build_options;
mod cargo_toml;
mod compile;
mod develop;
mod metadata;
mod module_writer;
mod python_interpreter;
#[cfg(feature = "upload")]
mod registry;
mod target;
#[cfg(feature = "upload")]
mod upload;
