<h1 align="center">key-list</h1>
<p align="center">A very minimal crate to get you a specific sequence between two characters in a string, fast.</p>


## How to use
```toml
[dependencies]
key-list = "1"
```

#### And some code
```rust
let text = "/this/ has some /keys/";
let list = KeyList::new(text, '/', '/');

for key in list {
    println!("{}", key); // '/this/', '/keys/'
}
```