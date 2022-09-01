use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

pub trait Matrix
where
    Self: Sized,
{
    type Item;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn size(&self) -> usize;
    fn get_coord(&self, coord: Coord) -> Option<Self::Item>;
    fn get_coord_mut(&mut self, coord: Coord) -> Option<&mut Self::Item>;
    fn transpose(&self) -> Self;
    fn neighbors(&self, coord: Coord, diagonals: bool) -> Vec<(Coord, Self::Item)>;
    fn neighbors_mut(&mut self, coord: Coord, diagonals: bool) -> Vec<(Coord, &mut Self::Item)>;
    fn flat_iter(&self) -> MatrixIterator<Self>;
    fn flat_iter_mut(&mut self) -> MatrixIteratorMut<Self>;

    fn values(&self) -> Vec<Self::Item> {
        self.flat_iter().map(|(_, v)| v).collect()
    }

    fn values_mut<'a>(&'a mut self) -> Vec<&'a mut Self::Item> {
        self.flat_iter_mut().map(|(_, v)| v).collect()
    }
}

impl<T: Copy> Matrix for Vec<Vec<T>> {
    type Item = T;

    fn width(&self) -> usize {
        self[0].len()
    }

    fn height(&self) -> usize {
        self.len()
    }

    fn size(&self) -> usize {
        self.width() * self.height()
    }

    fn get_coord(&self, coord: Coord) -> Option<T> {
        self.get(coord.y).and_then(|row| row.get(coord.x).copied())
    }

    fn get_coord_mut(&mut self, coord: Coord) -> Option<&mut Self::Item> {
        self.get_mut(coord.y).and_then(|row| row.get_mut(coord.x))
    }

    fn neighbors(&self, coord: Coord, diagonals: bool) -> Vec<(Coord, T)> {
        let Coord { x, y } = coord;
        (y.saturating_sub(1)..=y + 1)
            .flat_map(|ny| {
                (x.saturating_sub(1)..=x + 1)
                    .filter_map(|nx| {
                        let c = Coord { x: nx, y: ny };
                        ((nx != x || ny != y) && (diagonals || nx == x || ny == y))
                            .then_some(c)
                            .and_then(|c| self.get_coord(c))
                            .map(|v| (c, v))
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn neighbors_mut(&mut self, coord: Coord, diagonals: bool) -> Vec<(Coord, &mut T)> {
        let Coord { x, y } = coord;
        return MatrixIteratorMut::new_with_bounds(
            self,
            Coord {
                x: x.saturating_sub(1),
                y: y.saturating_sub(1),
            },
            Coord {
                x: (x + 2).min(self.width()),
                y: (y + 2).min(self.height()),
            },
        )
        .filter(|(c, _)| (c.x != x || c.y != y) && (diagonals || c.x == x || c.y == y))
        .collect();
    }

    fn transpose(&self) -> Vec<Vec<T>> {
        assert!(!self.is_empty());
        (0..self[0].len())
            .map(|j| self.iter().map(|row| row[j]).collect())
            .collect()
    }

    fn flat_iter(&self) -> MatrixIterator<Self> {
        MatrixIterator {
            matrix: self,
            pos: Coord { x: 0, y: 0 },
        }
    }

    fn flat_iter_mut(&mut self) -> MatrixIteratorMut<Self> {
        MatrixIteratorMut::new(self)
    }
}

pub struct MatrixIterator<'a, M>
where
    M: Matrix,
{
    matrix: &'a M,
    pos: Coord,
}

impl<'a, M> Iterator for MatrixIterator<'a, M>
where
    M: Matrix,
{
    type Item = (Coord, M::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.y >= self.matrix.height() {
            None
        } else {
            let ret = self.matrix.get_coord(self.pos).map(|v| (self.pos, v));
            self.pos.x += 1;
            if self.pos.x >= self.matrix.width() {
                self.pos.x = 0;
                self.pos.y += 1;
            }
            ret
        }
    }
}

pub struct MatrixIteratorMut<'a, M>
where
    M: Matrix,
{
    matrix: &'a mut M,
    pos: Coord,
    start: Coord,
    end: Coord,
}

impl<'a, M> MatrixIteratorMut<'a, M>
where
    M: Matrix,
{
    fn new(matrix: &'a mut M) -> Self {
        Self::new_with_bounds(
            matrix,
            Coord { x: 0, y: 0 },
            Coord {
                x: matrix.width(),
                y: matrix.height(),
            },
        )
    }

    fn new_with_bounds(matrix: &'a mut M, start: Coord, end: Coord) -> Self {
        Self {
            matrix,
            pos: start,
            start,
            end,
        }
    }
}

impl<'a, M> Iterator for MatrixIteratorMut<'a, M>
where
    M: Matrix,
{
    type Item = (Coord, &'a mut M::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.y >= self.end.y {
            None
        } else {
            unsafe {
                let v: *mut M::Item = self.matrix.get_coord_mut(self.pos).unwrap();
                self.pos.x += 1;
                if self.pos.x >= self.end.x {
                    self.pos.x = self.start.x;
                    self.pos.y += 1;
                }
                Some((self.pos, v.as_mut().unwrap()))
            }
        }
    }
}

#[test]
fn test_neighbors() {
    let xs = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    assert_eq!(
        xs.neighbors(Coord { x: 1, y: 1 }, true),
        vec![
            (Coord { x: 0, y: 0 }, 0),
            (Coord { x: 1, y: 0 }, 1),
            (Coord { x: 2, y: 0 }, 2),
            (Coord { x: 0, y: 1 }, 3),
            (Coord { x: 2, y: 1 }, 5),
            (Coord { x: 0, y: 2 }, 6),
            (Coord { x: 1, y: 2 }, 7),
            (Coord { x: 2, y: 2 }, 8),
        ]
    );
    assert_eq!(
        xs.neighbors(Coord { x: 1, y: 1 }, false),
        vec![
            (Coord { x: 1, y: 0 }, 1),
            (Coord { x: 0, y: 1 }, 3),
            (Coord { x: 2, y: 1 }, 5),
            (Coord { x: 1, y: 2 }, 7),
        ]
    );
}
