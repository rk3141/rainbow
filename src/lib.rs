use std::io::{self, Write};

use termcolor::{ ColorChoice, ColorSpec, StandardStream, WriteColor };
pub use termcolor::Color;

fn write_bgcolor(out: &str, color: Color, ) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_bg(Some(color)))?;
    write!(&mut stdout, "{}", out)?;
    stdout.reset()?;
    Ok(())
}

fn write_color(out: &str, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    write!(&mut stdout, "{}", out)?;
    stdout.reset()?;
    Ok(())
}

pub struct Rainbow
{
    colors: Vec<Color>
}

impl Rainbow
{
    pub fn custom(colors: Vec<Color>) -> Rainbow
    {
        Rainbow
        {
            colors: colors
        }
    }

    pub fn default() -> Rainbow
    {
        Rainbow
        {
            colors: vec![
                Color::Red,
                Color::Rgb(255,127,0),
                Color::Rgb(255,255,0),
                Color::Rgb(0,255,0),
                Color::Rgb(0,100,255),
                Color::Rgb(0,0,255),
                Color::Rgb(128,0,255),
            ]
        }
    }

    pub fn write_bg(&self, text: &str) -> io::Result<()>
    {
        let mut minus = 0;
        for (index,ch) in text.chars().enumerate()
        {
            if index-minus > self.colors.len()-1
            {
                minus += self.colors.len();
            }
        
            write_bgcolor(&format!("{}",ch), self.colors[
                index-minus
            ])?;
        }
        Ok(())
    }

    pub fn write(&self, text: &str) -> io::Result<()>
    {
        let mut minus = 0;
        for (index,ch) in text.chars().enumerate()
        {
            if index-minus > self.colors.len()-1
            {
                minus += self.colors.len();
            }
        
            write_color(&format!("{}",ch), self.colors[
                index-minus
            ])?;
        }
        Ok(())
    }
}