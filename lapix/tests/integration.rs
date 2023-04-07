#[cfg(feature = "test-utils")]
use lapix::TestImage;

use lapix::color::{BLACK, TRANSPARENT};
use lapix::{Color, Event, Point, Size, State};

#[cfg(feature = "test-utils")]
#[test]
fn empty_canvas() {
    let side = 10;
    let mut state = State::<TestImage>::new(Size::new(side, side), None, None);

    for i in 0..side {
        for j in 0..side {
            assert_eq!(state.canvas().pixel(Point::new(i, j)), TRANSPARENT);
        }
    }
}

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

#[cfg(feature = "test-utils")]
#[test]
fn draw_red_line() {
    let side = 10;
    let mut state = State::<TestImage>::new(Size::new(side, side), None, None);
    let red = Color::new(255, 0, 0, 255);
    state.execute(Event::SetMainColor(red));
    state.execute(Event::LineStart(Point::new(0, 0)));
    state.execute(Event::LineEnd(Point::new(side - 1, side - 1)));

    for i in 0..side {
        for j in 0..side {
            let color = if i == j { red } else { TRANSPARENT };

            assert_eq!(state.canvas().pixel(Point::new(i, j)), color);
        }
    }
}

#[cfg(feature = "test-utils")]
#[test]
fn draw_line_then_clear_canvas() {
    let side = 10;
    let mut state = State::<TestImage>::new(Size::new(side, side), None, None);
    state.execute(Event::LineStart(Point::new(0, 0)));
    state.execute(Event::LineEnd(Point::new(side - 1, side - 1)));
    state.execute(Event::ClearCanvas);

    for i in 0..side {
        for j in 0..side {
            assert_eq!(state.canvas().pixel(Point::new(i, j)), TRANSPARENT);
        }
    }
}

#[cfg(feature = "test-utils")]
#[test]
fn bucket() {
    let side = 10;
    let mut state = State::<TestImage>::new(Size::new(side, side), None, None);
    state.execute(Event::Bucket(Point::new(0, 0)));

    for i in 0..side {
        for j in 0..side {
            assert_eq!(state.canvas().pixel(Point::new(i, j)), BLACK);
        }
    }
}

#[cfg(feature = "test-utils")]
#[test]
fn bucket_then_erase() {
    let side = 10;
    let mut state = State::<TestImage>::new(Size::new(side, side), None, None);
    state.execute(Event::Bucket(Point::new(0, 0)));
    state.execute(Event::EraseStart);
    state.execute(Event::Erase(Point::new(0, 0)));
    state.execute(Event::Erase(Point::new(side - 1, side - 1)));
    state.execute(Event::EraseEnd);

    for i in 0..side {
        for j in 0..side {
            let color = if i == j { TRANSPARENT } else { BLACK };
            assert_eq!(state.canvas().pixel(Point::new(i, j)), color);
        }
    }
}
