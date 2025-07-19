# Ownership

- Rust's ownership system is a set of rules that governs memory management.
- It ensures that memory is automatically managed without needing a garbage collector.
- If any of the rules are violated, the code will not compile.

## Three Rules of Ownership
1. Each value in Rust has an owner, which is a variable that holds the value.
2. A value can only have one owner at a time.
3. When the owner goes out of scope, the value is dropped (memory is freed).

## Owner
The owner of a value is the variable or data structure that holds it and is responsible for allocating and freeing the memory used to store that data.

## Scope
The scope is the range within a program for which a variable is valid.

#### Global Scope
    It is accessible throughout the entire program.

#### Local Scope
    It is accessible only within a particular function or block of code.

    It is not accessible outside of that function or block.


## Copy vs. Move

```Rust
let x = 5; // x owns the value 5
let y = x; // y now owns the value 5, x is still valid
```
Here, the integer value of variable `x` will get copied to `y` and both `x` and `y` are usable, because i32 value has been "copied".

```Rust
let s1 = String::from("Hello"); // s1 owns the String
let s2 = s1; // s2 now owns the String, s1 is no longer valid
```

As `s1` is a `String`, which is just a pointer to data on the **heap**. Just the pointer will get copied to `s2`, NOT the whole data on the heap! At this point, `s1` and `s2` both point to the same data on the heap, but this violates the second rule of ownership, which states that a value can only have one owner at a time. Therefore, `s1` will be invalidated and cannot be used after assigning it to `s2`. This is done to avoid dangling pointers and ensure memory safety. Here, the ownership of the `String` is "moved" from `s1` to `s2`.

#### Deep Copy

```Rust
let s1 = String::from("Hello");
let s2 = s1.clone(); // s2 now owns a deep copy of the String

println!("s1: {}, s2: {}", s1, s2); // Both s1 and s2 are valid
```
In this case, `s1.clone()` creates a deep copy of the `String` data on the heap, allowing both `s1` and `s2` to be valid and usable independently. This is an explicit way to duplicate data in Rust, ensuring that both variables own their own copies of the data.

## Ownership and Functions

When passing ownership to functions, the ownership rules still apply. If a variable is passed to a function, it will be moved into that function unless explicitly cloned.

#### Example of Ownership Transfer


- ```Rust
    fn main() {
        let s = String::from("hello");

        takes_ownership(s); // s's value moves into the function...
        // ... and so is no longer valid here

        let x = 5;
        makes_copy(x); // x would still be valid here because i32 is Copy
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.


    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
    ```

- ```Rust
    fn main() {
    let s1 = gives_ownership();             // gives_ownership moves its return value into s1
    let s2 = String::from("hello");         // s2 comes into scope
    let s3 = takes_and_gives_back(s2);      // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    }

    fn gives_ownership() -> String {
        let some_string = String::from("yours");
        some_string // some_string is returned and moves out to the calling function
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string // a_string is returned and moves out to the calling function
    }
    ```

## Important Question
I don't have to care about ownership in another langauge and it seems to be complicating a simple issue. Why is it important in Rust?

Well, Rust's ownership system is designed to prevent memory safety issues such as:
- **Dangling Pointers**: Pointers that reference memory that has been freed.
- **Double Free**: Attempting to free the same memory more than once.
- **Memory Leaks**: Memory that is allocated but never freed, leading to increased memory usage over time.
- **Data Races**: Multiple threads accessing shared data without proper synchronization, leading to unpredictable behavior.


