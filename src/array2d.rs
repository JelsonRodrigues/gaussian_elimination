use std::{ops::{Index, IndexMut}, fmt::{Display, Debug}};


pub struct Array2D<T>{
    vec:Vec<T>,
    columns:usize,
    rows:usize,
}

impl<T> Array2D<T> {
    pub fn new(rows:usize, columns:usize) -> Self
    where 
        T: Default,
        T: Clone
    {
        let mut new = Array2D { vec: Vec::with_capacity(columns * rows), columns: columns, rows: rows };
        for _ in 0..(columns * rows) {
            new.vec.push(T::default());
        }
        return new;
    }

    pub fn insert(&mut self, row:usize, column:usize, value:T)
    where T: Clone
    {
        if Array2D::check_values(&self, row, column) {
            let index = row * self.columns + column;
            self.vec[index] = value.clone();
        }
        else {
            panic!("Index [{row}][{column}] is out of bounds [{}][{}]", self.rows, self.columns);
        }
    }
    pub fn insert_unchecked(&mut self, row:usize, column:usize, value:T) 
    where T: Clone
    {
        let index = row * self.columns + column;
        self.vec[index] = value.clone();
    }
    pub fn get(&self, row:usize, column:usize) -> &T
    {
        if Array2D::check_values(&self, row, column) {
            return &self.vec[self.get_index(row, column)];
        }
        else {
            panic!("Index [{row}][{column}] is out of bounds [{}][{}]", self.rows, self.columns);
        }
    }

    pub fn rows_len(&self) -> usize {
        return self.columns;
    }

    pub fn columns_len(&self) -> usize {
        return self.rows;
    }

    fn get_index(&self, row:usize, column:usize) -> usize {
        return row * self.rows + column;
    }
    fn check_values(&self, row:usize, column:usize) -> bool {
        return row < self.rows && column < self.columns;
    }
}


impl<T> Index<usize> for Array2D<T> 
{
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let index_start = index * self.columns;
        let index_end = (index + 1) * self.columns;
        
        &self.vec[index_start..index_end]
    }
}

impl<T> Index<(usize, usize)> for Array2D<T> 
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = index.0 * self.columns + index.1;

        &self.vec[index]
    }
}

impl<T> IndexMut<usize> for Array2D<T> 
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index_start = index * self.columns;
        let index_end = (index + 1) * self.columns;

        &mut self.vec[index_start..index_end]
    }
}

impl<T> IndexMut<(usize, usize)> for Array2D<T> 
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = index.0 * self.columns + index.1;
        &mut self.vec[index]
    }
}


pub struct Array2DIntoIterator<T> {
    array : Array2D<T>,
    index : usize
}

impl<T> Iterator for Array2DIntoIterator<T>
where T:Clone
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        let result = {
            if self.index >= self.array.rows_len() {
                None
            }
            else
            {
                Some(self.array[self.index].to_vec())
            }
        };
        self.index += 1;
        return result;
    }
}


impl<T> IntoIterator for Array2D<T> 
where T:Clone
{
    type Item = Vec<T>;
    type IntoIter = Array2DIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Array2DIntoIterator {
            array: self,
            index: 0
        }
    }
    
}

pub struct Array2DIterator<'a, T> {
    array : &'a Array2D<T>,
    index : usize
}

impl<'a, T> Iterator for Array2DIterator<'a, T>
where T:Clone
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<&'a [T]> {
        let result = {
            if self.index >= self.array.rows_len() {
                None
            }
            else
            {
                Some(&self.array[self.index])
            }
        };
        self.index += 1;
        return result;
    }
}


impl<'a, T> IntoIterator for &'a Array2D<T> 
where T:Clone
{
    type Item = &'a [T];
    type IntoIter = Array2DIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Array2DIterator {
            array: self,
            index: 0
        }
    }
    
}