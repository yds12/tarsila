use crate::Point;

pub fn distance(p1: Point<i32>, p2: Point<i32>) -> f32 {
    ((((p1.x - p2.x) as i64).pow(2) + ((p1.y - p2.y) as i64).pow(2)) as f64).sqrt() as f32
}

pub fn line(p1: Point<u16>, p2: Point<u16>) -> Vec<Point<u16>> {
    let mut line = Vec::new();

    let p1 = Point::new(p1.x as i32, p1.y as i32);
    let p2 = Point::new(p2.x as i32, p2.y as i32);

    let diff_x = p2.x - p1.x;
    let diff_y = p2.y - p1.y;
    let dist = distance(p1, p2);
    let dx = if dist < 0.1 { 0. } else { diff_x as f32 / dist };
    let dy = if dist < 0.1 { 0. } else { diff_y as f32 / dist };

    for i in 0..=dist.round() as usize {
        let x = (p1.x as f32 + (i as f32 * dx)).round() as u16;
        let y = (p1.y as f32 + (i as f32 * dy)).round() as u16;

        if let Some(Point { x: x0, y: y0 }) = line.last() {
            if x == *x0 && y == *y0 {
                continue;
            }
        }

        line.push((x, y).into());
    }

    line
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case((0,0), (2,0), vec![(0, 0), (1, 0), (2, 0)])]
    #[test_case((0,0), (0,2), vec![(0, 0), (0, 1), (0, 2)])]
    #[test_case((0,0), (2,2), vec![(0, 0), (1, 1), (2, 2)])]
    #[test_case((2,0), (0,0), vec![(0, 0), (1, 0), (2, 0)])]
    #[test_case((0,0), (3,1), vec![(0, 0), (1, 0), (2, 1), (3, 1)])]
    fn simple_line_cases(p1: (u16, u16), p2: (u16, u16), expected: Vec<(u16, u16)>) {
        let mut l = line(p1.into(), p2.into());
        l.sort();

        assert_eq!(l, expected.into_iter().map(Into::into).collect::<Vec<_>>());
    }

    #[test]
    fn odd_lines() {
        let p1 = (0, 0);
        let p2 = (2, 1);
        let expect = vec![(0, 0), (2, 1)];
        let either = vec![(1, 0), (1, 1)];
        let l = line(p1.into(), p2.into());

        for expected in expect {
            assert!(l.contains(&expected.into()));
        }

        assert!(l.contains(&either[0].into()) || l.contains(&either[1].into()));
    }
}
