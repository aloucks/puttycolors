pub const FOREGROUND: &'static str = "#CCCCCC";
pub const FOREGROUND_BOLD: &'static str = "#ffffff";
pub const BACKGROUND: &'static str = "#282C34";
pub const BACKGROUND_BOLD: &'static str = "#3E4451";
pub const CURSOR_TEXT: &'static str = "#e5e5e5";
pub const CURSOR_COLOR: &'static str = "#23d18b";
pub const ANSI_BLACK: &'static str = "#000000";
pub const ANSI_BRIGHT_BLACK: &'static str = "#666666";
pub const ANSI_RED: &'static str = "#cd3131";
pub const ANSI_BRIGHT_RED: &'static str = "#f14c4c";
pub const ANSI_GREEN: &'static str = "#0DBC79";
pub const ANSI_BRIGHT_GREEN: &'static str = "#23d18b";
pub const ANSI_YELLOW: &'static str = "#e5e510";
pub const ANSI_BRIGHT_YELLOW: &'static str = "#f5f543";
pub const ANSI_BLUE: &'static str = "#2472c8";
pub const ANSI_BRIGHT_BLUE: &'static str = "#3b8eea";
pub const ANSI_MAGENTA: &'static str = "#bc3fbc";
pub const ANSI_BRIGHT_MAGENTA: &'static str = "#d670d6";
pub const ANSI_CYAN: &'static str = "#11a8cd";
pub const ANSI_BRIGHT_CYAN: &'static str = "#29b8db";
pub const ANSI_WHITE: &'static str = "#e5e5e5";
pub const ANSI_BRIGHT_WHITE: &'static str = "#ffffff"; // "#e5e5e5";

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    assert_eq!("#", &hex[0..1], "hex strings must start with '#'");
    assert_eq!(7, hex.len(), "hex strings must be exactly 7 characters");
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
    (r, g, b)
}

#[test]
fn test_hex_to_rgb() {
    assert_eq!((41, 184, 219), hex_to_rgb("#29b8db"));
}

fn hex_to_rgb_csv(hex: &str) -> String {
    let (r, g, b) = hex_to_rgb(hex);
    format!("{},{},{}", r, g, b)
}

fn main() {
    println!("Windows Registry Editor Version 5.00");
    println!();
    println!(r"[HKEY_CURRENT_USER\Software\SimonTatham\PuTTY\Sessions\Default%20Settings]");
    println!("\"Colour0\"=\"{}\"", hex_to_rgb_csv(FOREGROUND));
    println!("\"Colour1\"=\"{}\"", hex_to_rgb_csv(FOREGROUND_BOLD));
    println!("\"Colour2\"=\"{}\"", hex_to_rgb_csv(BACKGROUND));
    println!("\"Colour3\"=\"{}\"", hex_to_rgb_csv(BACKGROUND_BOLD));
    println!("\"Colour4\"=\"{}\"", hex_to_rgb_csv(CURSOR_TEXT));
    println!("\"Colour5\"=\"{}\"", hex_to_rgb_csv(CURSOR_COLOR));
    println!("\"Colour6\"=\"{}\"", hex_to_rgb_csv(ANSI_BLACK));
    println!("\"Colour7\"=\"{}\"", hex_to_rgb_csv(ANSI_BRIGHT_BLACK));
    println!("\"Colour8\"=\"{}\"", hex_to_rgb_csv(ANSI_RED));
    println!("\"Colour9\"=\"{}\"", hex_to_rgb_csv(ANSI_BRIGHT_RED));
    println!("\"Colour10\"=\"{}\"", hex_to_rgb_csv(ANSI_GREEN));
    println!("\"Colour11\"=\"{}\"", hex_to_rgb_csv(ANSI_BRIGHT_GREEN));
    println!("\"Colour12\"=\"{}\"", hex_to_rgb_csv(ANSI_YELLOW));
    println!("\"Colour13\"=\"{}\"", hex_to_rgb_csv(ANSI_BRIGHT_YELLOW));
    println!("\"Colour14\"=\"{}\"", hex_to_rgb_csv(ANSI_BLUE));
    println!("\"Colour15\"=\"{}\"", hex_to_rgb_csv(ANSI_BRIGHT_BLUE));
    println!("\"Colour16\"=\"{}\"", hex_to_rgb_csv(ANSI_MAGENTA));
    println!("\"Colour17\"=\"{}\"", hex_to_rgb_csv(ANSI_BRIGHT_MAGENTA));
    println!("\"Colour18\"=\"{}\"", hex_to_rgb_csv(ANSI_CYAN));
    println!("\"Colour19\"=\"{}\"", hex_to_rgb_csv(ANSI_BRIGHT_CYAN));
    println!("\"Colour20\"=\"{}\"", hex_to_rgb_csv(ANSI_WHITE));
    println!("\"Colour21\"=\"{}\"", hex_to_rgb_csv(ANSI_BRIGHT_WHITE));
}
