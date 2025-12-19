# THE RUSTY PROJECT

## Key Lessons

### Chapter 1
- Rust executables can be run anywhere even if rust is not installed!
- Cargo is a package manager and build tool for rust programs
- Cargo commands: `cargo new|init|build|run|check`

### Chapter 2
- The guessing game

### Chapter 3
- Shadowing allows us to transform a value and return to the original value as soon as the transformations have ended or are out of scope
- Rust is statically typed
- Overflows causes a value to wrap around thereby leading to an unintended value. E.g When 257 is assigned to a u8, the value becomes 1 which may be unexpected.
- These methods are used to explicitly handle possible overflows

| Function Family | What it does | Example |
| --------------- | ------------ | ------- |
| wrapping_*      | supports wrapping | wrapping_add |
| checked_*       | returns `None` if there's overflow | checked_add|
| overflowing_*   | returns the wrapped value and a bool indicating whether an overflow occured | overflowing_add |
|saturating_*     | saturates at the value's min or max values | saturating_add |

- all floating point types are signed f32|f64
- A unit is an empty tuple `let unit: () = ()` and is the value returned implicitly by expressions that don't return any other value.

- if conditions must be given a condition that returns a boolean value
  > let num = 3
  > 
  > if num { ..would not work...}
  > if num < 3 { ... works ... }


