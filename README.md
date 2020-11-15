# Rainbow

### Functions

`write_rainbow_bg(text: &str)`
`write_rainbow_fg(text: &str)`

## Example

```rust
use std::io;

fn main() -> io::Result<()>
{
    rainbow::write_rainbow_fg("Hello, World")?;
    Ok(())
}
```