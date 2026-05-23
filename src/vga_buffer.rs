// src/vga_buffer.rs

// vga_buffer a simple text buffer for the VGA text mode
// vga means video graphics array
// vga buffer is 25 rows and 80 columns
// source: https://os.phil-opp.com/vga-text-mode/
// https://wiki.osdev.org/VGA_Hardware#Text_Mode

// importing required crates
use core::fmt; // fmt for formating
use lazy_static::lazy_static; // lazy for static varbles
use spin::Mutex; // spin for locking
use volatile::Volatile; // volatile for memory access



const BUFFER_HEIGHT: usize = 25; // height of the buffer
const BUFFER_WIDTH: usize = 80; // width of the buffer
const VGA_BUFFER_ADDRESS: usize = 0xb8000; // address of the buffer

lazy_static! { // lazy static for global access
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(VGA_BUFFER_ADDRESS as *mut Buffer) },
    });
}

#[allow(dead_code)] // allow dead code for unsed varbles
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // derive debug, clone, copy, partial eq, eq
#[repr(u8)] // repr u8 for enum

// color enum
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // repr transparent for struct
struct ColorCode(u8); // u8 for color code

impl ColorCode { // impl for color code
    const fn new(foreground: Color, background: Color) -> ColorCode { // const fn for new color code
        ColorCode((background as u8) << 4 | (foreground as u8)) // background << 4 | foreground
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // repr C for struct
struct ScreenChar { // struct for screeen char
    ascii_character: u8, // ascii character
    color_code: ColorCode, // color code
}

#[repr(transparent)] // repr transparnt
struct Buffer { // struct for buffer
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT], // chars for buffer
}

pub struct Writer { // struct for writer
    column_position: usize, // column position
    color_code: ColorCode, // color code
    buffer: &'static mut Buffer, // buffer
}

impl Writer { // impl for writer
    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.column_position = 0;
    }

    pub fn write_byte(&mut self, byte: u8) { // write byte
        match byte {// match byte
            b'\n' => self.new_line(),// new line
            byte => { // byte
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) { // write string
        for byte in s.bytes() { // for each byte
            match byte { // match byte
                0x20..=0x7e | b'\n' => self.write_byte(byte), // write byte
                _ => self.write_byte(0xfe), // write byte
            }
        }
    }

    pub fn write_at(&mut self, row: usize, col: usize, s: &str) { // write at function for specific position
        if row >= BUFFER_HEIGHT || col >= BUFFER_WIDTH { // if row or column is out of bounds
            return;
        }

        for (offset, byte) in s.bytes().enumerate() { // for each byte
            let current_col = col + offset;
            if current_col >= BUFFER_WIDTH { // if current column is out of bounds chekcing
                break;
            }

            let ascii_character = match byte {
                0x20..=0x7e => byte,
                _ => 0xfe,
            };

            self.buffer.chars[row][current_col].write(ScreenChar {
                ascii_character,
                color_code: self.color_code,
            });
        }
    }

    pub fn write_centered(&mut self, row: usize, s: &str) { // write center
        let text_width = s.len().min(BUFFER_WIDTH); // text width
        let col = (BUFFER_WIDTH - text_width) / 2; // column
        self.write_at(row, col, s); // write at
    }

    fn new_line(&mut self) { // new line
        for row in 1..BUFFER_HEIGHT { // for each row
            for col in 0..BUFFER_WIDTH { // for each column
                let character = self.buffer.chars[row][col].read(); // read character
                self.buffer.chars[row - 1][col].write(character); // write character to previous row
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1); 
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) { // clear row
        let blank = ScreenChar {
            ascii_character: b' ', // space character
            color_code: self.color_code, // color code
        };

        for col in 0..BUFFER_WIDTH { // for each column
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer { // impl write for writer
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s); // write string
        Ok(()) // return ok. error handling not implemented yet
    }
}

#[macro_export] // macro for printing
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*))); // print with arguments
}

#[macro_export] // macro for printing
macro_rules! println { // println macro rules
    () => ($crate::vga_buffer::_print(format_args!("\n"))); // println with no arguments
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!("{}\n", format_args!($($arg)*)))); // println with arguments
}

#[doc(hidden)] // hidden from documentation
pub fn _print(args: fmt::Arguments) { // print arguments
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[test_case] // test println simple
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case] // test println many
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

////////////////////////////////////////////
// end of file                            //
////////////////////////////////////////////

