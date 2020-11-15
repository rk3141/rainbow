use std::io::{self, Write};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn write_bgcolor(out: &str, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_bg(Some(color)))?;
    write!(&mut stdout, "{}", out)?;
    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Black)))?;
    Ok(())
}

fn write_color(out: &str, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    write!(&mut stdout, "{}", out)?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    Ok(())
}

pub fn write_rainbow_fg(string: &str) -> io::Result<()> {
    let colors = vec![
        Color::Red,
        Color::Rgb(255,127,0),
        Color::Rgb(255,255,0),
        Color::Rgb(0,255,0),
        Color::Rgb(0,100,255),
        Color::Rgb(0,0,255),
        Color::Rgb(128,0,255),
    ];

    
    let mut minus = 0;
    for (index,ch) in string.chars().enumerate()
    {
        if index-minus > colors.len()-1
        {
           minus += colors.len(); 
        }
        
        write_color(&format!("{}",ch), colors[
            index-minus
        ])?;
    }
    Ok(())
}

pub fn write_rainbow_bg(string: &str) -> io::Result<()> {
    let colors = vec![
        Color::Red,
        Color::Rgb(255,127,0),
        Color::Rgb(255,255,0),
        Color::Rgb(0,255,0),
        Color::Rgb(0,100,255),
        Color::Rgb(0,0,255),
        Color::Rgb(128,0,255),
    ];

    
    let mut minus = 0;
    for (index,ch) in string.chars().enumerate()
    {
        if index-minus > colors.len()-1
        {
           minus += colors.len(); 
        }
        
        write_bgcolor(&format!("{}",ch), colors[
            index-minus
        ])?;
    }
    Ok(())
}


// write_bgcolor(&format!("{}",ch), colors[
//     index-minus
// ])?;
