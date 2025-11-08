#[derive(Debug, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn adjacent(&self, width_bound: usize, height_bound: usize) -> Vec<Coord> {
        let mut a = Vec::with_capacity(4);
        if self.x > 0 {
            a.push(Coord {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            a.push(Coord {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x < width_bound - 1 {
            a.push(Coord {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y < height_bound - 1 {
            a.push(Coord {
                x: self.x,
                y: self.y + 1,
            });
        }
        a
    }

    pub fn adjecent_with_diagonals(&self, width_bound: usize, height_bound: usize) -> Vec<Coord> {
        let mut a = Vec::with_capacity(8);
        if self.x > 0 {
            a.push(Coord {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            a.push(Coord {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.x < width_bound - 1 {
            a.push(Coord {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y < height_bound - 1 {
            a.push(Coord {
                x: self.x,
                y: self.y + 1,
            });
        }
        if self.y > 0 && self.x > 0 {
            a.push(Coord {
                x: self.x - 1,
                y: self.y - 1,
            });
        }
        if self.x < width_bound - 1 && self.y > 0 {
            a.push(Coord {
                x: self.x + 1,
                y: self.y - 1,
            });
        }
        if self.x > 0 && self.y < height_bound - 1 {
            a.push(Coord {
                x: self.x - 1,
                y: self.y + 1,
            });
        }
        if self.x < width_bound - 1 && self.y < height_bound - 1 {
            a.push(Coord {
                x: self.x + 1,
                y: self.y + 1,
            });
        }
        a
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

// pub struct Map<T> {
//     map: Rc<[T]>,
//     width: usize,
//     height: usize,
// }
//
// impl<T: Copy> Map<T> {
//     fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = T>> {
//         (0..self.height).map(move |y| (0..self.width).map(move |x| self[Coord { x, y }]))
//     }
//     fn cols(&self) -> impl Iterator<Item = impl Iterator<Item = T>> {
//         (0..self.width).map(move |x| (0..self.height).map(move |y| self[Coord { x, y }]))
//     }
//
// }
//
// impl<T> Index<Coord> for Map<T> {
//     type Output = T;
//
//     fn index(&self, index: Coord) -> &Self::Output {
//         &self.map[index.y * self.width + index.x]
//     }
// }
//
// struct MapNotSquareError(())
//
// impl<T: Clone> TryFrom<&[Vec<T>]> for Map<T> {
//     type Error = MapNotSquareError;
//
//     fn try_from(value: &[Vec<T>]) -> Result<Self, Self::Error> {
//         if value.len() == 0 {
//             return Ok(Map {
//                 map: Rc::new([]),
//                 width : 0,
//                 height: 0
//             });
//         }
//         let width = value[0].len();
//         if !value.iter().all(|row| row.len() == width) {
//             return Err(MapNotSquareError);
//         }
//
//         let height = value.len();
//         let map = Rc::new(value.concat());
//         Ok(Map {
//             map,
//             width,
//             height
//         })
//     }
// }
//
//     // fn from_iter<I: IntoIterator<Item = &[T]>(iter: I) -> Self {
//     //     let mut width = Some(())
//     //     let map: [T] = iter.into_iter().fold(Vec::new(), |acc, row| acc = [acc, row].concat());
//     //     Map {
//     //         map: Rc::new(map),
//     //         width:
//     //     }
//     // }
