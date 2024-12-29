# incremynt
Lettering numbers and incrementing it.


## Usage
### Deployed on GitHub Pages
- https://hayas1.github.io/incremynt/
```
┏━┛┃
┗━┓┃┏━━┓┏━━┓┏┓┏┓
┏━┛┃┃┏┓┃┗━┓┃┃┃┃┃
┗━━┛┃┃┃┃┏━┛┃┃┗┛┃
┏━━┓┃┃┃┃┃┏━┛┗━┓┃
┗━┓┃┃┗┛┃┃┗━┓  ┃┃
┏━┛┃┗━━┛┗━━┛  ┗┛
┃┏━┛
```
The next year is ready to begin.

### CLI
```sh
$ cargo install --git https://github.com/hayas1/incremynt incremynt-cli
$ incremynt-cli
┏━┛┃
┗━┓┃┏━━┓┏━━┓┏┓┏┓
┏━┛┃┃┏┓┃┗━┓┃┃┃┃┃
┗━━┛┃┃┃┃┏━┛┃┃┗┛┃
┏━━┓┃┃┃┃┃┏━┛┗━┓┃
┗━┓┃┃┗┛┃┃┗━┓  ┃┃
┏━┛┃┗━━┛┗━━┛  ┗┛
┃┏━┛
$ cargo uninstall incremynt-cli
```
The next year is ready to begin.

### Library
```toml
[dependencies]
incremynt = { git = "https://github.com/hayas1/incremynt" }
```

```rust
use incremynt::increment::Incremynt;
fn main() {
    println!("{}", Incremynt::new(2024.into(), 3024.into()));
}
```

```sh
$ cargo run
┏━┛┃
┗━┓┃┏━━┓┏━━┓┏┓┏┓
┏━┛┃┃┏┓┃┗━┓┃┃┃┃┃
┗━━┛┃┃┃┃┏━┛┃┃┗┛┃
┏━━┓┃┃┃┃┃┏━┛┗━┓┃
┗━┓┃┃┗┛┃┃┗━┓  ┃┃
┏━┛┃┗━━┛┗━━┛  ┗┛
┃┏━┛
```
The next year is ready to begin.
