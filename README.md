# rust-security-framework

[![Latest Version](https://img.shields.io/crates/v/security-framework.svg)](https://lib.rs/crates/security-framework)

[Documentation](https://kornelski.github.io/rust-security-framework/doc/security_framework/)

Bindings to the Apple's `Security.framework`. Allows use of TLS and Keychain from Rust.

## Bindings

Topics implemented, in reference to [Apple's Security Framework documentation](https://developer.apple.com/documentation/security?language=objc):

* Authorization and Authentication
  * [ ] Authorization Plug-ins
  * [x] Authorization Services, except:
    * [ ] AuthorizationCopyRightsAsync
  * [ ] ~Password AutoFill~
  * [ ] Sessions
  * [ ] Shared Web Credentials
* Secure Data
  * [x] Keychain Services
* Secure Code
  * [ ] Code Signing Services
* Cryptography
  * [ ] ASN.1
  * [ ] Cryptographic Message Syntax Services
  * [x] Randomization Services
  * [x] Security Transforms
* Legacy Interfaces
  * [ ] Common Security Services Manager
  * [x] Secure Transport
  * [ ] Secure Download

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

