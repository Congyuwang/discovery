//! logic to compute dot movement.
const STARTING_POINT: (i8, i8) = (0, 2);
pub const CENTER: (i8, i8) = (2, 2);

const CLOCKWISE_IN_0: (i8, i8) = (0, 1);
const CLOCKWISE_IN_1: (i8, i8) = (1, 1);
const CLOCKWISE_OUT_0: (i8, i8) = (1, 2);
const CLOCKWISE_OUT_1: (i8, i8) = CENTER;

const COUNTER_CLOCKWISE_IN_0: (i8, i8) = (0, 3);
const COUNTER_CLOCKWISE_IN_1: (i8, i8) = (1, 3);
const COUNTER_CLOCKWISE_OUT_0: (i8, i8) = (1, 2);
const COUNTER_CLOCKWISE_OUT_1: (i8, i8) = CENTER;

const PX_MASK: i8 = 0b0000_0111;
const META_MASK: i8 = 0b1000_0000u8 as i8;

/// State of the dot.
///
/// state.0 < 0: counter_clockwise,
/// state.1 < 0: going_out
pub struct DotState {
    state: (i8, i8),
}

impl DotState {
    /// create a new dot at starting point.
    pub const fn new() -> Self {
        Self {
            state: STARTING_POINT,
        }
    }

    /// the coordinate of the dot.
    #[inline]
    pub fn px(&self) -> (i8, i8) {
        (self.state.0 & PX_MASK, self.state.1 & PX_MASK)
    }

    /// spiral the dot.
    ///
    /// by default does nothing on `STARTING_POINT` and `CENTER`.
    pub fn spiral<SCB: FnMut(&mut Self), CB: FnMut(&mut Self)>(
        &mut self,
        mut starting_point_cb: SCB,
        mut center_cb: CB,
    ) {
        let px = &self.px();
        match px {
            &CENTER => center_cb(self),
            &STARTING_POINT => starting_point_cb(self),
            _ => (),
        }
        match (self.is_clockwise(), self.is_going_in()) {
            (true, true) => self.spiral_in_cw(px),
            (true, false) => self.spiral_out_cw(px),
            (false, true) => self.spiral_in_ccw(px),
            (false, false) => self.spiral_out_ccw(px),
        }
    }

    #[inline]
    pub fn is_clockwise(&self) -> bool {
        self.state.0 & META_MASK == 0
    }

    #[inline]
    pub fn is_going_in(&self) -> bool {
        self.state.1 & META_MASK == 0
    }

    #[inline]
    pub fn toggle_clockwise(&mut self) {
        self.state.0 ^= META_MASK;
    }

    #[inline]
    pub fn toggle_going_in(&mut self) {
        self.state.1 ^= META_MASK;
    }

    #[inline]
    pub fn inc_x(&mut self) {
        self.state.0 += 1;
    }

    #[inline]
    pub fn inc_y(&mut self) {
        self.state.1 += 1;
    }

    #[inline]
    pub fn dec_x(&mut self) {
        self.state.0 -= 1;
    }

    #[inline]
    pub fn dec_y(&mut self) {
        self.state.1 -= 1;
    }

    /// spiral in clockwise
    fn spiral_in_cw(&mut self, px: &(i8, i8)) {
        match px {
            &CLOCKWISE_IN_0 | &CLOCKWISE_IN_1 => self.go_in_cw(),
            &CENTER => (),
            _ => self.next_px_cw(px),
        }
    }

    /// spiral in counter-clockwise
    fn spiral_in_ccw(&mut self, px: &(i8, i8)) {
        match px {
            &COUNTER_CLOCKWISE_IN_0 | &COUNTER_CLOCKWISE_IN_1 => self.go_in_ccw(),
            &CENTER => (),
            _ => self.next_px_ccw(px),
        }
    }

    /// spiral out clockwise
    fn spiral_out_cw(&mut self, px: &(i8, i8)) {
        match px {
            &CLOCKWISE_OUT_0 | &CLOCKWISE_OUT_1 => self.go_out_cw(),
            &STARTING_POINT => (),
            _ => self.next_px_cw(px),
        }
    }

    /// spiral out counter-clockwise
    fn spiral_out_ccw(&mut self, px: &(i8, i8)) {
        match px {
            &COUNTER_CLOCKWISE_OUT_0 | &COUNTER_CLOCKWISE_OUT_1 => self.go_out_ccw(),
            &STARTING_POINT => (),
            _ => self.next_px_ccw(px),
        }
    }

    /// assume that px == CLOCKWISE_IN_0 | CLOCKWISE_IN_1
    #[inline]
    fn go_in_cw(&mut self) {
        self.inc_x();
        self.inc_y();
    }

    /// assume that px == CLOCKWISE_OUT_0 | CLOCKWISE_OUT_1
    #[inline]
    fn go_out_cw(&mut self) {
        self.dec_x();
        self.inc_y();
    }

    /// assume that px == COUNTER_CLOCKWISE_IN_0 | COUNTER_CLOCKWISE_IN_1
    #[inline]
    fn go_in_ccw(&mut self) {
        self.inc_x();
        self.dec_y();
    }

    /// assume that px == COUNTER_CLOCKWISE_OUT_0 | COUNTER_CLOCKWISE_OUT_1
    #[inline]
    fn go_out_ccw(&mut self) {
        self.dec_x();
        self.dec_y();
    }

    /// clockwise
    fn next_px_cw(&mut self, px: &(i8, i8)) {
        match px {
            // top && !right
            (0, 0..=3) => self.inc_y(),
            // right && !bottom
            (0..=3, 4) => self.inc_x(),
            // bottom && !left
            (4, 1..=4) => self.dec_y(),
            // left && !top
            (1..=4, 0) => self.dec_x(),

            // inner layer

            // top && !right
            (1, 1) | (1, 2) => self.inc_y(),
            // right && !bottom
            (1, 3) | (2, 3) => self.inc_x(),
            // bottom && !left
            (3, 2) | (3, 3) => self.dec_y(),
            // left && !top
            (2, 1) | (3, 1) => self.dec_x(),

            // unreachable, including center, negative numbers, out of range cases
            _ => unreachable!(),
        }
    }

    /// counter-clockwise
    fn next_px_ccw(&mut self, px: &(i8, i8)) {
        match px {
            // left && !bottom
            (0..=3, 0) => self.inc_x(),
            // bottom && !right
            (4, 0..=3) => self.inc_y(),
            // right && !top
            (1..=4, 4) => self.dec_x(),
            // top && !left
            (0, 1..=4) => self.dec_y(),

            // inner layer

            // left && !bottom
            (1, 1) | (2, 1) => self.inc_x(),
            // bottom && !right
            (3, 1) | (3, 2) => self.inc_y(),
            // right && !top
            (2, 3) | (3, 3) => self.dec_x(),
            // top && !left
            (1, 2) | (1, 3) => self.dec_y(),

            // unreachable, including center, negative numbers, out of range cases
            _ => unreachable!(),
        }
    }
}
