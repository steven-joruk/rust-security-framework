//! Wrappers around the OSX Security Framework.

#![warn(missing_docs)]
#![allow(non_upper_case_globals)]

#[cfg(any(target_os="ios", target_os="macos"))]
extern crate security_framework_sys;
#[cfg(any(target_os="ios", target_os="macos"))]
#[macro_use]
extern crate core_foundation;
#[cfg(any(target_os="ios", target_os="macos"))]
extern crate core_foundation_sys;
#[cfg(any(target_os="ios", target_os="macos"))]
extern crate libc;

#[cfg(test)]
extern crate hex;
#[cfg(test)]
extern crate tempdir;

#[cfg(target_os = "macos")]
use os::macos::access::SecAccess;
#[cfg(target_os = "macos")]
use os::macos::keychain::SecKeychain;

#[cfg(test)]
macro_rules! p {
    ($e:expr) => {
        match $e {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        }
    };
}

#[cfg(all(not(feature = "OSX_10_13"), feature = "alpn"))]
#[macro_use]
mod dlsym;

#[cfg(any(target_os="ios", target_os="macos"))] pub mod base;
#[cfg(any(target_os="ios", target_os="macos"))] pub mod certificate;
#[cfg(any(target_os="ios", target_os="macos"))] pub mod cipher_suite;
#[cfg(any(target_os="ios", target_os="macos"))] pub mod identity;
#[cfg(any(target_os="ios", target_os="macos"))] pub mod import_export;
#[cfg(any(target_os="ios", target_os="macos"))] pub mod item;
#[cfg(any(target_os="ios", target_os="macos"))] pub mod key;
#[cfg(any(target_os="ios", target_os="macos"))] pub mod os;
#[cfg(any(target_os="ios", target_os="macos"))] pub mod policy;
#[cfg(any(target_os="ios", target_os="macos"))] pub mod random;
#[cfg(any(target_os="ios", target_os="macos"))] pub mod secure_transport;
#[cfg(any(target_os="ios", target_os="macos"))] pub mod trust;

#[cfg(target_os = "macos")]
trait Pkcs12ImportOptionsInternals {
    fn keychain(&mut self, keychain: SecKeychain) -> &mut Self;
    fn access(&mut self, access: SecAccess) -> &mut Self;
}

#[cfg(target_os = "macos")]
trait ItemSearchOptionsInternals {
    fn keychains(&mut self, keychains: &[SecKeychain]) -> &mut Self;
}

trait AsInner {
    type Inner;
    fn as_inner(&self) -> Self::Inner;
}

#[cfg(any(target_os="ios", target_os="macos"))]
fn cvt(err: core_foundation_sys::base::OSStatus) -> base::Result<()> {
    match err {
        security_framework_sys::base::errSecSuccess => Ok(()),
        err => Err(base::Error::from_code(err)),
    }
}

#[cfg(test)]
mod test {
    use certificate::SecCertificate;

    pub fn certificate() -> SecCertificate {
        let certificate = include_bytes!("../test/server.der");
        p!(SecCertificate::from_der(certificate))
    }
}
