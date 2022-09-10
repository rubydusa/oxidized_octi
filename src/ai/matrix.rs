use std::error::Error;
use std::path::Path;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::super::board::Position;

#[derive(Clone, Serialize, Deserialize)]
pub struct Matrix<T> {
    arr: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    // constructors

    pub fn new(arr: Vec<T>, width: usize, height: usize) -> Matrix<T> {
        if arr.len() != width * height {
            panic!("Invalid dimensions: {}x{} {}", width, height, arr.len());
        }

        Matrix { arr, width, height }
    }

    // getters

    pub fn get(&self, pos: &Position) -> Option<&T> {
        let (x, y) = (pos.x() as usize, pos.y() as usize);
        self.arr.get(Matrix::<T>::construct_index(x, y, self.width))
    }
    pub fn arr(&self) -> &Vec<T> {
        &self.arr
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.arr.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.arr.iter_mut()
    }

    // private

    fn construct_index(x: usize, y: usize, w: usize) -> usize {
        x + y * w
    }
}

impl<T: FromStr> Matrix<T>
where
    <T as FromStr>::Err: 'static + Error,
{
    pub fn from_csv<P: AsRef<Path>>(p: P) -> Result<Matrix<T>, Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(p)?;
        let mut arr = Vec::<T>::new();
        let mut height = 0;

        for result in reader.records() {
            let record = result?;
            for item in record.iter() {
                arr.push(item.parse()?);
            }
            height += 1;
        }

        if height == 0 {
            height = 1;
        }

        let width = arr.len() / height;

        Ok(Matrix { arr, height, width })
    }
}
