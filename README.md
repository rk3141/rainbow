# Rainbow

A crate for writing text in rainbow colors.

# Examples

### Write text with foreground colors
```rust
use rainbow_text::{ Foreground, Rainbow };

fn main() -> std::io::Result<()>
{
    let rain = Rainbow::default("Hello, World");
    
    rain.write()?;
    
    Ok(())
}
```

### Write text with background colors
```rust
use rainbow_text::{ Background, Rainbow };

fn main() -> std::io::Result<()>
{
    let rain = Rainbow::default("Hello, World");
    
    rain.write_bg()?;
    
    Ok(())
}
```

### Change the text
```rust
use rainbow_text::{ ChangeText, Foreground, Rainbow };

fn main() -> std::io::Result<()>
{
    let mut rain = Rainbow::default("Hello, World");
    
    rain.write()?;
    rain.change_text("Rainbows!");
    rain.write()?;
    
    Ok(())
}
```

