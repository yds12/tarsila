#[cfg(feature = "test-utils")]
use lapix::TestImage;

use lapix::color::{BLACK, TRANSPARENT};
use lapix::{Color, Event, Point, Size, State};

#[cfg(feature = "test-utils")]
#[test]
fn draw_line() {
    let side = 10;
    let mut state = State::<TestImage>::new(Size::new(side, side), None, None);
    state.execute(Event::LineStart(Point::new(0, 0)));
    state.execute(Event::LineEnd(Point::new(side - 1, side - 1)));

    for i in 0..side {
        for j in 0..side {
            let color = if i == j { BLACK } else { TRANSPARENT };

            assert_eq!(state.canvas().pixel(Point::new(i, j)), color);
        }
    }
}
