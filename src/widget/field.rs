use crate::usym::hbox;
use soyo::{
    tui::{Letter, Quad},
    widget::Render,
};

pub struct Field {
    w: i32,
    h: i32,
    sw: i32,
    sh: i32,
}

impl Field {
    pub fn new(w: i32, h: i32, sw: i32, sh: i32) -> Self {
        Self { w, h, sw, sh }
    }

    pub fn get_wh(&self) -> (i32, i32) {
        let Self { w, h, sw, sh } = self;
        (w * (sw + 1) + 1, h * (sh + 1) + 1)
    }

    fn render_intersect(&self, quad: Quad, letter: &mut Letter) {
        if quad.x == 0 {
            if quad.y == 0 {
                *letter.c = hbox::CTL;
            } else if quad.y == quad.h - 1 {
                *letter.c = hbox::CBL;
            } else {
                *letter.c = hbox::IVL;
            }
        } else if quad.x == quad.w - 1 {
            if quad.y == 0 {
                *letter.c = hbox::CTR;
            } else if quad.y == quad.h - 1 {
                *letter.c = hbox::CBR;
            } else {
                *letter.c = hbox::IVR;
            }
        } else if quad.y == 0 {
            *letter.c = hbox::IHT;
        } else if quad.y == quad.h - 1 {
            *letter.c = hbox::IHB;
        } else {
            *letter.c = hbox::CRX;
        }
    }

    fn render_horizon_line(&self, letter: &mut Letter) {
        *letter.c = hbox::LNH;
    }

    fn render_vertical_line(&self, letter: &mut Letter) {
        *letter.c = hbox::LNV;
    }
}

impl Default for Field {
    fn default() -> Self {
        Self {
            w: 7,
            h: 3,
            sw: 20,
            sh: 12,
        }
    }
}

impl Render for Field {
    fn render(&self, quad: Quad, letter: &mut Letter) {
        let v = quad.x % (self.sw + 1) == 0;
        let h = quad.y % (self.sh + 1) == 0;

        if h && v {
            self.render_intersect(quad, letter);
        } else if h {
            self.render_horizon_line(letter);
        } else if v {
            self.render_vertical_line(letter);
        }
    }
}
