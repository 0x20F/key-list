<h1 align="center">key-list</h1>
<p align="center">A very minimal crate to get you a specific sequence between two characters in a string, fast.</p>

<p align="center">
   <img alt="Build status badge" src="https://github.com/0x20F/key-list/workflows/build/badge.svg"/>
   <img alt="Crates io badge" src="https://img.shields.io/crates/v/key-list.svg"/>
   <a href="https://www.gnu.org/licenses/gpl-3.0"><img alt="License badge" src="https://img.shields.io/github/license/0x20F/key-list"/></a>
</p>


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