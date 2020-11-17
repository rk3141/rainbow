# Rainbow

A crate for writing text in rainbow colors.

# Examples

### Write text with foreground colors
```rust
use rainbow_text::Rainbow;

fn main() -> std::io::Result<()>
{
    let rain = Rainbow::default();
    
    rain.write("Hello, World")?;
    
    Ok(())
}
```

### Write text with background colors
```rust
use rainbow_text::Rainbow;

fn main() -> std::io::Result<()>
{
    let rain = Rainbow::default();
    
    rain.write_bg("Hello, World")?;
    
    Ok(())
}
```

### Custom Rainbows!
```rust
use rainbow_text::{ Rainbow, Color };

fn main() -> std::io::Result<()>
{
    let rain = Rainbow::custom(
        vec![
            Color::Rgb(255,0,0),
            Color::Rgb(0,255,0),
            Color::Rgb(0,0,255),
        ]
    );
    
    rain.write("Hello, World")?;
    
    Ok(())
}
```