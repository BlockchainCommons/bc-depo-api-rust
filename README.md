# Blockchain Commons Depository API

<!--Guidelines: https://github.com/BlockchainCommons/secure-template/wiki -->

### _by Wolf McNally_

## Introduction

Blockchain Commons Depository (`depo`) is server-based software that provides
for the secure storage and retrieval of binary objects without regard to their
contents.

This crate (`depo-api`) provides a Rust API for the `depo` server. It is
intended to be used by clients that want to store and retrieve BLOBs from a
`depo` server. It primarily includes structures and methods to marshal and
unmarshal the API requests and responses.

## Status

This is an early development release. It is not yet ready for production use.

Please see the [`depo`](https://crates.io/crates/depo) crate for more
information.

## Version History

### 0.14.0 - July 3, 2025
- Align to dependencies.
- Improve code formatting and consistency.
