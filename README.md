# Email Syntax Verification Optimizer

Ultra-fast, zero-allocation email validation library optimized for enterprise production use.

## Features

- **Ultra-fast performance**: Optimized with SIMD operations, lookup tables, and unsafe optimizations
- **Zero allocations**: No heap allocations during validation
- **Memory efficient**: Minimal memory footprint with static lookup tables
- **Enterprise ready**: Production-tested with comprehensive error handling
- **RFC compliant**: Supports RFC 5322 email syntax validation
- **IP literal support**: Validates IPv4 and IPv6 address literals
- **IDN support**: International domain name validation
- **Multiple interfaces**: Trait-based and direct function APIs

## Performance

This library is designed for maximum performance:
- Static lookup tables for character validation
- SIMD-optimized byte processing
- Branch prediction optimizations
- Unsafe operations where safe
- Minimal function call overhead

## Usage

```rust
use email_syntax_verify_opt::{validate_email, ValidateEmail};

// Direct function call
assert!(validate_email("test@example.com"));

// Trait usage
assert!("user@domain.co.uk".validate_email());
assert!("invalid.email".validate_email() == false);

// Byte slice validation
let email_bytes = b"test@example.com";
assert!(email_bytes.validate_email());
```

## Benchmarks

Run benchmarks with:
```bash
cargo bench
```

## License

MIT OR Apache-2.0