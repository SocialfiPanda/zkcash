# ZKCash Test Suite

This test suite provides comprehensive testing for the ZKCash program, a privacy-focused application built on Solana. The tests cover both unit testing of individual components and integration testing of the program's instructions in a simulated on-chain environment.

## Test Structure

The test suite is organized into the following structure:

```
tests/
├── common/
│   ├── mod.rs
│   ├── test_utils.rs
│   └── fixtures.rs
├── unit/
│   ├── mod.rs
│   ├── poseidon_tests.rs
│   ├── merkle_tree_tests.rs
│   └── utils_tests.rs
├── integration/
│   ├── mod.rs
│   ├── initialize_tests.rs
│   ├── shield_tests.rs
│   ├── withdraw_tests.rs
│   └── error_tests.rs
└── lib.rs
```

## Test Coverage

### Unit Tests

- **Poseidon Hash Tests**: Tests for the Poseidon hash functions used in the ZKCash program, including hash_1, hash_2, hash_left_right, and compute_merkle_root.
- **Merkle Tree Tests**: Tests for the Merkle tree implementation, including initialization, leaf insertion, and capacity limits.
- **Utility Tests**: Tests for utility functions like PDA derivation and byte conversion.

### Integration Tests

- **Initialize Tests**: Tests for the Initialize instruction, including successful initialization and error cases.
- **Shield Tests**: Tests for the Shield instruction, including successful shielding and error cases.
- **Withdraw Tests**: Tests for the Withdraw instruction, including successful withdrawal and error cases.
- **Error Tests**: Specific tests for all error conditions defined in the program.

## Setup Instructions

1. Place the test files in your project's `program/tests` directory.

2. Add the following to your `program/Cargo.toml` file:

```toml
[[test]]
name = "zkcash_tests"
path = "tests/lib.rs"

[dev-dependencies]
solana-program-test = "2.2.1"
solana-sdk = "2.2.1"
tokio = { version = "1.14", features = ["full"] }
```

3. Run the tests with:

```bash
cd program
cargo test
```

## Notes on Test Implementation

- The tests use mock data for proofs and cryptographic elements to avoid the complexity of generating real ZK proofs in tests.
- Some tests for the Withdraw instruction are designed to fail with specific errors since generating valid proofs would require a full ZK proving system.
- The test utilities provide helper functions for setting up test environments, creating accounts, and processing transactions.

## Customization

You may need to customize the tests for your specific implementation:

- Update the program ID in `fixtures.rs` to match your deployed program.
- Adjust the mock data in `fixtures.rs` to match your expected inputs and outputs.
- If your instruction formats differ, update the instruction creation functions in `test_utils.rs`.

## Solana 2.2.1 Compatibility

These tests are designed for Solana 2.2.1 and use the latest APIs and features. If you're using a different version, you may need to adjust the tests accordingly.
