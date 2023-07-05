use std::{
    iter::StepBy,
    slice::{Iter, IterMut},
};

pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        let mut data = Vec::with_capacity(width * height);
        data.resize_with(width * height, T::default);

        Self {
            width,
            height,
            data,
        }
    }

    pub fn init(width: usize, height: usize, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            width,
            height,
            data: vec![value; width * height],
        }
    }

    pub fn from_vec(vec: Vec<T>, width: usize) -> Self {
        assert_eq!(vec.len() % width, 0);

        Self {
            width,
            height: vec.len() / width,
            data: vec,
        }
    }

    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T {
        self.data.get_unchecked(x + y * self.width)
    }

    pub unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.data.get_unchecked_mut(x + y * self.width)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(x + y * self.width)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(x + y * self.width)
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn rows(&self) -> usize {
        self.height
    }

    pub fn cols(&self) -> usize {
        self.width
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.width = 0;
        self.height = 0;
        self.data.clear();
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub fn iter_col(&self, col: usize) -> StepBy<Iter<T>> {
        self.data[col..].iter().step_by(self.width)
    }

    pub fn iter_col_mut(&mut self, col: usize) -> StepBy<IterMut<T>> {
        self.data[col..].iter_mut().step_by(self.width)
    }

    pub fn iter_row(&self, row: usize) -> Iter<T> {
        let start = row * self.width;
        self.data[start..(start + self.width)].iter()
    }

    pub fn iter_row_mut(&mut self, row: usize) -> IterMut<T> {
        let start = row * self.width;
        self.data[start..(start + self.width)].iter_mut()
    }

    pub fn flatten(&self) -> &Vec<T> {
        &self.data
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.data.fill(value);
    }

    pub fn fill_with(&mut self, func: impl FnMut() -> T) {
        self.data.fill_with(func);
    }

    pub fn map<U>(self, func: impl FnMut(T) -> U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            data: self.data.into_iter().map(func).collect(),
        }
    }
}

impl<T> Grid<Option<T>> {
    pub fn or(self, other: Self) -> Self {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);

        Self {
            width: self.width,
            height: self.height,
            data: self
                .into_vec()
                .into_iter()
                .zip(other.into_vec().into_iter())
                .map(|(a, b)| a.or(b))
                .collect(),
        }
    }
}
