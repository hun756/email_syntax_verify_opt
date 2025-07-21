# Email Syntax Verification Optimizer

Ultra-fast, zero-allocation email validation library optimized for enterprise production use.

## 🚀 Features

- **Ultra-fast performance**: Optimized with SIMD operations, lookup tables, and unsafe optimizations
- **Zero allocations**: No heap allocations during validation
- **Memory efficient**: Minimal memory footprint with static lookup tables
- **Enterprise ready**: Production-tested with comprehensive error handling
- **RFC compliant**: Supports RFC 5322 email syntax validation
- **IP literal support**: Validates IPv4 and IPv6 address literals
- **IDN support**: International domain name validation
- **Multiple interfaces**: Trait-based and direct function APIs

## 📊 Performance Benchmarks

Benchmarks performed on **AMD Ryzen 7 7800X3D (8-core)** processor:

### Validation Performance Comparison

| Test Case | Our Validator | Validator Crate | Regex | Performance Gain |
|-----------|---------------|-----------------|-------|------------------|
| **Valid Emails (15 items)** | `310.94 ns` | `2.0242 µs` | `585.92 ns` | **6.5x faster** |
| **Invalid Emails (16 items)** | `164.85 ns` | `5.1443 µs` | `363.16 ns` | **31x faster** |
| **Realistic Emails (10 items)** | `242.67 ns` | `1.4587 µs` | `435.53 ns` | **6x faster** |
| **Single Email** | `17.509 ns` | `132.44 ns` | `41.271 ns` | **7.5x faster** |

### Email Length Impact Analysis

| Email Length | Our Validator | Validator Crate | Performance Gain |
|--------------|---------------|-----------------|------------------|
| **Short** (`a@b.co`) | `8.8229 ns` | `99.775 ns` | **11x faster** |
| **Medium** (`user.name@example.com`) | `16.487 ns` | `135.29 ns` | **8x faster** |
| **Long** (67 chars) | `46.331 ns` | `207.98 ns` | **4.5x faster** |

### Key Performance Highlights

- 🏆 **Up to 31x faster** than popular validator crate
- ⚡ **2-3x faster** than regex-based validation
- 🎯 **Consistent performance** across different email lengths
- 💾 **Zero heap allocations** during validation
- 🔥 **Sub-nanosecond per character** processing speed

## 🛠 Technical Optimizations

This library achieves exceptional performance through:

- **Static lookup tables** for O(1) character validation
- **SIMD-optimized byte processing** with 8-byte chunks
- **Branch prediction optimizations** with `#[cold]` annotations
- **Strategic unsafe operations** for bounds checking elimination
- **Memory-aligned data structures** for cache efficiency
- **Minimal function call overhead** with aggressive inlining

## 📖 Usage

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

// String validation
let email_string = String::from("user@example.org");
assert!(email_string.validate_email());
```

## 🧪 Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench -- "single_email_comparison"

# Generate HTML reports
cargo bench -- --output-format html
```

## 📈 Use Cases

Perfect for high-performance applications requiring:

- **Web form validation** with millions of requests
- **Batch email processing** and cleaning
- **Real-time email filtering** systems
- **API input validation** with strict latency requirements
- **Data pipeline processing** with high throughput needs

## 🔬 Benchmark Details

All benchmarks use Criterion.rs with:
- 100 iterations per measurement
- Statistical outlier detection
- Warm-up cycles for CPU optimization
- Black-box optimization prevention

## 📄 License

MIT OR Apache-2.0
