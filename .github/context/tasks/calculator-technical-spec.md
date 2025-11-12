# Technical Specification: Calculator Functions

## Architecture Overview
Create a calculator module in Rust with mathematical utility functions for addition and subtraction.

## File Structure

```
src/
├── main.rs          (existing - will add module declaration)
└── calculator.rs    (new - calculator functions module)
```

## Technical Implementation

### Module: `calculator.rs`

Location: `src/calculator.rs`

#### Function: `add`
```rust
/// Adds two i32 numbers and returns their sum.
///
/// # Arguments
/// * `a` - The first number
/// * `b` - The second number
///
/// # Returns
/// The sum of `a` and `b`
///
/// # Examples
/// ```
/// use agent_panopticon::calculator::add;
/// assert_eq!(add(5, 3), 8);
/// assert_eq!(add(-1, 1), 0);
/// ```
pub fn add(a: i32, b: i32) -> i32
```

#### Function: `subtract`
```rust
/// Subtracts the second number from the first and returns the result.
///
/// # Arguments
/// * `a` - The number to subtract from
/// * `b` - The number to subtract
///
/// # Returns
/// The difference of `a` minus `b`
///
/// # Examples
/// ```
/// use agent_panopticon::calculator::subtract;
/// assert_eq!(subtract(5, 3), 2);
/// assert_eq!(subtract(0, 5), -5);
/// ```
pub fn subtract(a: i32, b: i32) -> i32
```

## Testing Strategy

### Test Module
Location: `src/calculator.rs` (inline `#[cfg(test)] mod tests`)

#### Test Cases for `add`:
1. **Positive numbers** - Standard addition
2. **Zero handling** - Adding zero, adding to zero
3. **Negative numbers** - Both operands negative
4. **Mixed signs** - Positive + negative combinations
5. **Boundary cases** - Large values approaching i32::MAX

#### Test Cases for `subtract`:
1. **Positive numbers** - Standard subtraction
2. **Zero handling** - Subtracting zero, subtracting from zero
3. **Negative numbers** - Both operands negative
4. **Mixed signs** - Positive - negative, negative - positive
5. **Boundary cases** - Large value differences

## Integration

### Update `main.rs`:
```rust
pub mod calculator;

fn main() {
    println!("Hello, members of the panopticon!");
    println!("Calculator Demo:");
    println!("5 + 3 = {}", calculator::add(5, 3));
    println!("5 - 3 = {}", calculator::subtract(5, 3));
}
```

## Implementation Order

1. **Builder**: Create `src/calculator.rs` with both functions
2. **Forge**: Add comprehensive test suite to `src/calculator.rs`
3. **Builder**: Update `src/main.rs` to declare the module
4. **Atlas**: Execute tests with `cargo test`
5. **Herald**: Report test results
6. **Refiner**: Review and improve code quality
7. **Sentinel**: Validate complete solution

## Success Criteria

- Both functions implemented correctly
- All tests pass (minimum 8 test cases)
- Code is well-documented
- Module properly integrated into main.rs

---
**Created by:** Oracle  
**Date:** 12 November 2025
