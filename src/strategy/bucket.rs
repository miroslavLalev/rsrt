pub struct Bucket {
    x: u32,
    y: u32,
    count: u32,

    current_x: u32,
    current_y: u32,
    done: bool,
}

impl Bucket {
    pub fn new(x: u32, y: u32, count: u32) -> Bucket {
        Bucket {
            x,
            y,
            count,
            current_x: 0,
            current_y: 0,
            done: false,
        }
    }
}

impl Iterator for Bucket {
    type Item = Vec<(u32, u32)>;

    fn next(&mut self) -> Option<Self::Item> {
        let count_sqrt = (self.count as f64).sqrt();
        let bx = (self.x as f64 / count_sqrt).ceil() as u32; // valid because of self.x type
        let by = (self.y as f64 / count_sqrt).ceil() as u32; // valid because of self.y type

        if self.done {
            return None;
        }

        let mut result = Vec::new();
        for y in 1..=by {
            let curr_y = self.current_y * by + (y - 1);
            if curr_y >= self.y {
                continue;
            }

            for x in 1..=bx {
                let curr_x = self.current_x * bx + (x - 1);
                if curr_x >= self.x {
                    continue;
                }

                result.push((curr_y, curr_x))
            }
        }

        self.current_x += 1;
        if self.current_x * bx >= self.x {
            self.current_x = 0;
            self.current_y += 1;
            if self.current_y * by >= self.y {
                self.done = true;
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buckets_fitting() {
        let mut bucket = Bucket::new(10, 10, 4);
        assert_eq!(
            bucket.next(),
            Some(vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (2, 0),
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
                (4, 0),
                (4, 1),
                (4, 2),
                (4, 3),
                (4, 4),
            ])
        );

        assert_eq!(
            bucket.next(),
            Some(vec![
                (0, 5),
                (0, 6),
                (0, 7),
                (0, 8),
                (0, 9),
                (1, 5),
                (1, 6),
                (1, 7),
                (1, 8),
                (1, 9),
                (2, 5),
                (2, 6),
                (2, 7),
                (2, 8),
                (2, 9),
                (3, 5),
                (3, 6),
                (3, 7),
                (3, 8),
                (3, 9),
                (4, 5),
                (4, 6),
                (4, 7),
                (4, 8),
                (4, 9),
            ])
        );

        assert_eq!(
            bucket.next(),
            Some(vec![
                (5, 0),
                (5, 1),
                (5, 2),
                (5, 3),
                (5, 4),
                (6, 0),
                (6, 1),
                (6, 2),
                (6, 3),
                (6, 4),
                (7, 0),
                (7, 1),
                (7, 2),
                (7, 3),
                (7, 4),
                (8, 0),
                (8, 1),
                (8, 2),
                (8, 3),
                (8, 4),
                (9, 0),
                (9, 1),
                (9, 2),
                (9, 3),
                (9, 4),
            ])
        );

        assert_eq!(
            bucket.next(),
            Some(vec![
                (5, 5),
                (5, 6),
                (5, 7),
                (5, 8),
                (5, 9),
                (6, 5),
                (6, 6),
                (6, 7),
                (6, 8),
                (6, 9),
                (7, 5),
                (7, 6),
                (7, 7),
                (7, 8),
                (7, 9),
                (8, 5),
                (8, 6),
                (8, 7),
                (8, 8),
                (8, 9),
                (9, 5),
                (9, 6),
                (9, 7),
                (9, 8),
                (9, 9),
            ])
        );

        assert_eq!(bucket.next(), None);
        assert_eq!(bucket.next(), None);
        assert_eq!(bucket.next(), None);
    }
}
