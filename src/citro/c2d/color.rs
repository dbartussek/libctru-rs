
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Color(pub u32);

const fn u2f(v: u8) -> f32 {
    (v as f32) / (u8::MAX as f32)
}
const fn f2u(v: f32) -> u8 {
    let v = if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    };
    (v * (u8::MAX as f32) + 0.5) as u8
}

macro_rules! create_accessor {
    (
        $get: ident,
        $get_float: ident,
        $set: ident,
        $set_float: ident,
        $with: ident,
        $with_float: ident,
        $component: ident
    ) => {
        pub const fn $get(&self) -> u8 {
            self.get(Self::$component)
        }
        pub const fn $get_float(&self) -> f32 {
            u2f(self.$get())
        }
        pub fn $set(&mut self, v: u8) {
            *self = self.$with(v);
        }
        pub fn $set_float(&mut self, v: f32) {
            self.$set(f2u(v))
        }
        pub const fn $with(self, v: u8) -> Self {
            self.with(Self::$component, v)
        }
        pub const fn $with_float(self, v: f32) -> Self {
            self.$with(f2u(v))
        }
    };
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl Color {
    pub const A: usize = 24;
    pub const B: usize = 16;
    pub const G: usize = 8;
    pub const R: usize = 0;

    create_accessor!(r, rf, set_r, set_rf, with_r, with_rf, R);

    create_accessor!(g, gf, set_g, set_gf, with_g, with_gf, G);

    create_accessor!(b, bf, set_b, set_bf, with_b, with_bf, B);

    create_accessor!(a, af, set_a, set_af, with_a, with_af, A);

    pub const fn new(r: u8, b: u8, g: u8) -> Self {
        Self::new_alpha(r, b, g, u8::MAX)
    }

    pub const fn new_alpha(r: u8, b: u8, g: u8, a: u8) -> Self {
        Color(0).with_r(r).with_b(b).with_g(g).with_a(a)
    }

    pub const fn new_float(r: f32, b: f32, g: f32) -> Self {
        Self::new_alpha_float(r, b, g, 1.0)
    }

    pub const fn new_alpha_float(r: f32, b: f32, g: f32, a: f32) -> Self {
        Color(0).with_rf(r).with_bf(b).with_gf(g).with_af(a)
    }

    const fn get(&self, component: usize) -> u8 {
        ((self.0 >> component) & 0xFF) as u8
    }

    const fn with(mut self, component: usize, value: u8) -> Self {
        self.0 = (self.0 & !(0xFF << component)) | ((value as u32) << component);
        self
    }
}
