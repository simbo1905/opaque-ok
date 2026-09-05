# opaque-ok

A set of defaults for configuring [opaque-ke](https://github.com/facebook/opaque-ke) (RFC 9807). Derived from [tiptenbrink/opaque-borink](https://github.com/tiptenbrink/opaque-borink).

OPAQUE is an augmented PAKE: the server authenticates a password it never learns, and the stored credential is not usable for offline precomputation attacks.

## Defaults

| Role | Choice |
|---|---|
| OPRF group | Ristretto255 ([`curve25519-dalek`](https://github.com/dalek-cryptography/curve25519-dalek) via [`voprf`](https://github.com/facebook/voprf), SHA-512 hashing) |
| Key exchange | TripleDH over Ristretto255 |
| Password hashing | Argon2 (default parameters) |
| Encoding | Base64url, unpadded |

## Layout

* [`opaque-borink`](./opaque-borink) — core Rust library
* [`opaquepy`](./opaquepy) — Python bindings ([maturin](https://github.com/PyO3/maturin))
* [`opaquewasm`](./opaquewasm) — WASM bindings for JavaScript

## Breaking changes

Serializing formats are not stable across major upgrades. From `opaque-borink` 0.7.0 (opaque-ke v4, RFC 9807):

* `ServerSetup` values from 0.6.x or lower fail to deserialize; recreate them
* password files deserialize unchanged

## Test

```
cd opaque-borink && cargo test
cd opaquewasm && wasm-pack test --headless --chrome
cd opaquepy && uv run --with maturin maturin develop && uv run pytest
```

## License

Apache-2.0 OR MIT. Copyright 2022-2026 Tip ten Brink.
