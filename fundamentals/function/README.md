# functions

Functions in Rust are declared using the **fn** keyword.Functions can take multiple input parameters and return a value.

The return value of the function is denoted by an arrow **->**. If a function does not return a value, the return type is not specified.

```rust
fn add(a: i32, b: i32)-> i32 {
     a + b;
}

```

## Function Overloading

Rust does not support traditional function overloading (functions with the same name but different parameter types). However, this can often be achieved using traits and generics.

### Generic Function Declaration

```rust
fn double<T: std::ops::Add<Output = T> + Copy>(x: T) -> T {
    x + x
}
```

a. **fn double**:
Defines a function named double.

b. **<T>**
The angle brackets define a generic type parameter T. This allows the function to work with multiple types instead of being restricted to a specific one, such as i32 or f64.

c. T **std::ops::Add<Output = T>**
This is a trait bound. It specifies constraints on the type T:

**std::ops::Add** Requires T to implement the Add trait, which is responsible for the + operator.
Output = T: Ensures that the result of T + T is also of type T. This is important for consistency.
d. **+ Copy**
Another trait bound that requires T to implement the Copy trait. This ensures that T is a type that can be copied rather than moved (e.g., integers, floats, but not String).

e. **(x: T)**
The parameter x is of type T.

f. **-> T**
Specifies that the function returns a value of type T.

g. **x + x**

This is the body of the function. It doubles the input by adding it to itself. The + operator is valid because of the Add trait bound.

## Summary

- Functions in Rust are declared with fn.

- Parameters are specified as name: type.

- Functions can return values using the -> syntax.

- The last expression in a function is returned implicitly unless the return keyword is explicitly used.

- Functions without a return type implicitly return () (unit type).
  
Rust does not allow traditional function overloading but uses traits and generics to achieve similar behavior.

>Functions are a powerful tool in Rust, enabling concise, reusable, and type-safe code. The language's emphasis on clear and explicit function definitions ensures safety and correctness