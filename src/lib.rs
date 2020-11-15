use std::io::{self, Write};

use termcolor::{ ColorChoice, ColorSpec, StandardStream, WriteColor };
pub use termcolor::Color;

fn write_bgcolor(out: &str, color: Color) -> io::Result<()> {
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

pub trait Background
{
    fn write_bg(&self) -> io::Result<()>;
}

pub trait Foreground
{
    fn write(&self) -> io::Result<()>;
}

pub trait ChangeText
{
    fn change_text(&mut self,  new_text: &str);
}

pub struct Rainbow
{
    text: String,
    colors: Vec<Color>
}

impl Rainbow
{
    pub fn custom(text: &str, colors: Vec<Color>) -> Rainbow
    {
        Rainbow
        {
            text: String::from(text),
            colors: colors
        }
    }

    pub fn default(text: &str) -> Rainbow
    {
        Rainbow
        {
            text: String::from(text),
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
}

impl Background for Rainbow
{
    fn write_bg(&self) -> io::Result<()>
    {
        let mut minus = 0;
        for (index,ch) in self.text.chars().enumerate()
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
}

impl Foreground for Rainbow
{
    fn write(&self) -> io::Result<()>
    {
        let mut minus = 0;
        for (index,ch) in self.text.chars().enumerate()
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

impl ChangeText for Rainbow
{
    fn change_text(&mut self, new_text: &str)
    {
        self.text = String::from(new_text);
    }
}