use crate::view::Render;
use soyo::tui::{Color, Letter, Quad};
use std::char::{from_digit, from_u32};
use unic_ucd_category::GeneralCategory;
use unicode_bidi::bidi_class;

#[derive(Default)]
pub struct Utable {
    cell: u8,
}

impl Utable {
    pub fn set_cell(&mut self, cell: u8) {
        self.cell = cell;
    }

    pub fn get_wh(&self) -> (i32, i32) {
        (6 + 32, 17)
    }

    fn render_row_title(&self, quad: Quad, letter: &mut Letter) {
        let row = (quad.y - 1) as u32;
        let cell = self.cell as u32;
        *letter.c = if quad.y > 0 {
            match quad.x {
                0 => 'U',
                1 => '+',
                2 => from_digit(cell / 16, 16).unwrap_or('\0'),
                3 => from_digit(cell % 16, 16).unwrap_or('\0'),
                4 => from_digit(row % 16, 16).unwrap_or('\0'),
                5 => '0',
                _ => ' ',
            }
        } else {
            ' '
        }
    }

    fn render_col_title(&self, quad: Quad, letter: &mut Letter) {
        let col = (quad.x - 6) as u32;
        *letter.c = if col % 2 == 0 {
            ' '
        } else {
            from_digit(col / 2, 16).unwrap_or('\0')
        }
    }

    fn render_item(&self, quad: Quad, letter: &mut Letter) {
        *letter.c = if quad.x % 2 == 0 {
            '\0'
        } else {
            let row = quad.y - 1;
            let col = (quad.x - 6) / 2;
            let code = self.get_code(row as u32, col as u32);

            let (c, fg) = Self::map_basic(code);
            *letter.fg = fg;
            c
        }
    }

    fn get_code(&self, row: u32, col: u32) -> u32 {
        let cell = self.cell as u32;
        cell * 0x100 + row * 0x10 + col
    }

    fn map_basic(code: u32) -> (char, Color) {
        if let Some(c) = from_u32(code) {
            use GeneralCategory::*;
            match GeneralCategory::of(c) {
                NonspacingMark | SpacingMark | EnclosingMark => ('M', Color::GREEN),
                SpaceSeparator | LineSeparator | ParagraphSeparator => ('S', Color::GREEN),
                Control => ('C', Color::RED),
                Format => ('F', Color::RED),
                Surrogate => ('S', Color::RED),
                PrivateUse => ('P', Color::GRAY),
                Unassigned => ('U', Color::GRAY),
                _ => {
                    use unicode_bidi::BidiClass::*;
                    match bidi_class(c) {
                        L | EN | ES | ET | CS | ON => (c, Color::WHITE),
                        R => ('R', Color::NAVY),
                        AL | AN => ('A', Color::NAVY),
                        B | S | WS => ('N', Color::NAVY),
                        LRE | LRO | RLE | RLO | PDF | LRI | RLI | FSI | PDI => ('X', Color::NAVY),
                        _ => ('E', Color::NAVY),
                    }
                }
            }
        } else {
            ('U', Color::RED)
        }
    }
}

impl Render for Utable {
    fn render(&self, quad: Quad, letter: &mut Letter) {
        *letter.bg = Color::BLACK;
        *letter.fg = Color::WHITE;
        if quad.x < 6 {
            self.render_row_title(quad, letter)
        } else if quad.y < 1 {
            self.render_col_title(quad, letter)
        } else {
            self.render_item(quad, letter)
        }
    }
}
