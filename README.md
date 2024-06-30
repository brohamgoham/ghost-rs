# Ghost
`Ghost` is a tool for removing your executable while it's process is still running.  



## Usage

```rust
// On windows (`svcmsrpc`)
use ghost;

fn main() {
    match ghost::ninja() {
        Ok(_) => println!("Just went GHOST 👻"),
        Err(e) => println!("Nope! => {}", e),
    };
}
```

```rust
// With placeholder 
use ghost::ninja;

fn main() {
    #[cfg(target_os = "windows")]
    match ghost::ninja_with_placeholder("temporary") {
        Ok(_) => println!("Went GHOST!!"),
        Err(e) => println!("Nope! => {}", e),
    };
}
```