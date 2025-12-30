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
  >
  > if num < 3 { ... works ... }

- `break value;` can be used to return a value from a loop
- loops can be labelled and break statement can be called on a labelled loop from an inner loop.
  ```
  'loop_name: loop { ... }
  break 'loop_name;
  ```
- ```For``` loops are more efficient when iterating over elements of a collection

### Chapter 4 -> Ownership
- Each value has an owner.
- There can be only one owner at a time
- When the owner goes out of scope, the value will be dropped
- Types that are stored on the heap cannot be shallow copied, they can however be cloned.
- Ownership moves around as the value is passed into functions
- References allow another variable to borrow a value without claiming ownership.
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.
- We can take string slices using syntax ```&s[startIndex..endIndex]```
- String slices and string literals have same type which is `&str`
