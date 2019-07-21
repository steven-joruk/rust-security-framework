#![allow(bad_style)]

#[cfg(any(target_os="ios", target_os="macos"))]
extern crate core_foundation_sys;

#[cfg_attr(any(target_os="ios", target_os="macos"), link(name = "Security", kind = "framework"))]
extern "C" {}

#[cfg(target_os = "macos")]
pub mod access;
#[cfg(any(target_os="ios", target_os="macos"))]
pub mod base;
#[cfg(any(target_os="ios", target_os="macos"))]
pub mod certificate;
#[cfg(target_os = "macos")]
pub mod certificate_oids;
#[cfg(any(target_os="ios", target_os="macos"))]
pub mod cipher_suite;
#[cfg(target_os = "macos")]
pub mod digest_transform;
#[cfg(target_os = "macos")]
pub mod encrypt_transform;
#[cfg(any(target_os="ios", target_os="macos"))]
pub mod identity;
#[cfg(any(target_os="ios", target_os="macos"))]
pub mod import_export;
#[cfg(any(target_os="ios", target_os="macos"))]
pub mod item;
#[cfg(any(target_os="ios", target_os="macos"))]
pub mod key;
#[cfg(target_os = "macos")]
pub mod keychain;
#[cfg(target_os = "macos")]
pub mod keychain_item;
#[cfg(any(target_os="ios", target_os="macos"))]
pub mod policy;
#[cfg(any(target_os="ios", target_os="macos"))]
pub mod random;
#[cfg(any(target_os="ios", target_os="macos"))]
pub mod secure_transport;
#[cfg(target_os = "macos")]
pub mod transform;
#[cfg(any(target_os="ios", target_os="macos"))]
pub mod trust;
