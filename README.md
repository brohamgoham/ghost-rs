# Ghost
`Ghost` is a rust library that allows you to delete your executable while it's running.  



## Usage

```rust
// With a default placeholder value on windows (`svcmsrpc`)
use ghost;

fn main() {
    match ghost::ninja() {
        Ok(_) => println!("Just went GHOST 👻"),
        Err(e) => println!("Nope! => {}", e),
    };
}
```

```rust
// With a placeholder you provide
use ghost::ninja;

fn main() {
    #[cfg(target_os = "windows")]
    match ghost::ninja_with_placeholder("temporary") {
        Ok(_) => println!("Went GHOST!!"),
        Err(e) => println!("Nope! => {}", e),
    };
}
```