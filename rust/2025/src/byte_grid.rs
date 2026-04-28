use std::ops::{Index, IndexMut};

use anyhow::{Result, anyhow};
use memchr::memchr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid<T> {
    data: Vec<T>,
    height: usize,
    width: usize,
}

impl Grid<u8> {
    pub fn from_input_str(s: &str) -> Result<Self> {
        Grid::from_input_lines(s.lines())
    }

    pub fn from_input_lines<'a>(it: impl IntoIterator<Item = &'a str>) -> Result<Self> {
        let mut lines = it.into_iter().peekable();
        let width = lines.peek().map_or(0, |l| l.len());
        let mut height = 0;
        let mut data: Vec<u8> =
            Vec::with_capacity(lines.size_hint().1.map_or(width, |x| x * width));
        for line in lines {
            if line.len() != width {
                return Err(anyhow!("Grid is ragged"));
            }
            height += 1;
            data.extend_from_slice(line.as_bytes());
        }
        Ok(Grid {
            data,
            height,
            width,
        })
    }
    pub fn find(&self, val: u8) -> Option<(usize, usize)> {
        memchr(val, self.data.as_slice()).map(|idx| (idx / self.width, idx % self.width))
    }
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self {
        Grid {
            data: vec![fill; width * height],
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if col >= self.width {
            return None;
        }
        self.data.get(row * self.width + col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if col >= self.width {
            return None;
        }
        self.data.get_mut(row * self.width + col)
    }

    pub fn enumerate(&self) -> impl DoubleEndedIterator<Item = ((usize, usize), &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, x)| ((i / self.width, i % self.width), x))
    }

    pub fn neighbours(
        &self,
        row: usize,
        col: usize,
    ) -> impl DoubleEndedIterator<Item = (usize, usize)> + 'static {
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let height = self.height;
        let width = self.width;

        OFFSETS.into_iter().filter_map(move |(a, b)| {
            let row = row.checked_add_signed(a).filter(|&x| x < height);
            let col = col.checked_add_signed(b).filter(|&x| x < width);
            row.zip(col)
        })
    }

    pub fn col(&self, idx: usize) -> impl Iterator<Item = &T> {
        self.col_at_rows(idx, 0..self.height)
    }

    pub fn col_at_rows<I, II>(&self, col_idx: usize, rows: II) -> impl Iterator<Item = &T>
    where
        I: Iterator<Item = usize>,
        II: IntoIterator<IntoIter = I>,
    {
        rows.into_iter().map(move |row| &self[(row, col_idx)])
    }

    pub fn map<U>(&self, mut f: impl FnMut((usize, usize), &T) -> U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            data: self.enumerate().map(|(coord, x)| f(coord, x)).collect(),
        }
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.width..(index + 1) * self.width]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.width..(index + 1) * self.width]
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &T {
        self.get(index.0, index.1).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        self.get_mut(row, col).unwrap()
    }
}
