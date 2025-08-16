use super::*;

pub struct Tilemap<T> {
    width: usize,
    height: usize,
    tiles: Vec<T>,
}

impl<T> Tilemap<T> {
    pub fn new_empty() -> Tilemap<T> {
        Tilemap {
            width: 0,
            height: 0,
            tiles: vec![],
        }
    }
    pub fn new_with_fn(width: usize, height: usize, mut f: impl FnMut() -> T) -> Tilemap<T> {
        let mut tiles = Vec::with_capacity(width*height);
        for _y in 0 .. height {
            for _x in 0 .. width {
                tiles.push(f());
            }
        }
        Tilemap {
            width,
            height,
            tiles,
        }
    }
    pub fn new_with_coordinated_fn(width: usize, height: usize, mut f: impl FnMut(Point) -> T) -> Tilemap<T> {
        let mut tiles = Vec::with_capacity(width*height);
        for y in 0 .. height as i32 {
            for x in 0 .. width as i32 {
                tiles.push(f(Point { x , y }));
            }
        }
        Tilemap {
            width,
            height,
            tiles,
        }
    }
    pub fn new_with_row_iterators(rows: impl Iterator<Item=impl Iterator<Item=T>>) -> Tilemap<T> {
        let mut ret = Tilemap::new_empty();
        for row in rows {
            ret.add_row(row);
        }
        ret
    }
    /// If we are currently 0x0, make us contain this row (Wx1). If we already
    /// have rows, add this row and make sure it has the correct width. If
    /// shenanigans occur, PANIC!!!
    pub fn add_row(&mut self, row: impl Iterator<Item=T>) {
        self.tiles.extend(row);
        if self.height == 0 {
            self.width = self.tiles.len();
            self.height = 1;
        } else {
            self.height += 1;
            if self.width * self.height != self.tiles.len() {
                panic!("incorrectly-sized row added to existing tilemap");
            }
        }
    }
    fn coords_to_index(&self, coords: Point) -> Option<usize> {
        if coords.x < 0 || coords.y < 0 { return None }
        if coords.x as usize >= self.width || coords.y as usize >= self.height {
            return None
        }
        Some(coords.x as usize + (coords.y as usize * self.width))
    }
    /// If the coordinates are in range, return a reference to the
    /// tile at those coordinates. If out of range, return None.
    pub fn get_tile(&self, coords: Point) -> Option<&T> {
        Some(&self.tiles[self.coords_to_index(coords)?])
    }
    pub fn get_tile_mut(&mut self, coords: Point) -> Option<&mut T> {
        // Solra calls this the "vogon librarian" and didn't explain why
        let index = self.coords_to_index(coords)?;
        Some(&mut self.tiles[index])
    }
    pub fn set_tile(&mut self, coords: Point, new_value: T) -> Option<T> {
        let mut ret = new_value;
        std::mem::swap(self.get_tile_mut(coords)?, &mut ret);
        Some(ret)
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Return an iterator that will yield the coordinates of each tile,
    /// starting in the upper left and proceeding left to right, top to bottom.
    /// (Pairs nicely with `tiles`)    
    pub fn coords(&self) -> impl Iterator<Item=Point> {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| Point { x: x as i32, y: y as i32 }))
    }
    /// Return an iterator over the tiles of the tilemap, starting in the upper
    /// left and proceeding left to right, top to bottom
    pub fn tiles(&self) -> impl Iterator<Item=&T> {
        self.tiles.iter()
    }
}

impl<T> Tilemap<Option<T>> {
    pub fn new_with_nones(width: usize, height: usize) -> Tilemap<Option<T>> {
        Self::new_with_fn(width, height, || None)
    }
}

impl<T: Default> Tilemap<T> {
    pub fn new_with_defaults(width: usize, height: usize) -> Tilemap<T> {
        Self::new_with_fn(width, height, T::default)
    }
}

impl<T: Clone> Tilemap<T> {
    pub fn new_with_clones(width: usize, height: usize, mother: T) -> Tilemap<T> {
        Self::new_with_fn(width, height, || mother.clone())
    }
}

impl<T: Copy> Tilemap<T> {
    pub fn new_with_copies(width: usize, height: usize, mother: T) -> Tilemap<T> {
        Self::new_with_clones(width, height, mother)
    }
}

impl Tilemap<char> {
    pub fn new_with_chars_from_stdin() -> Tilemap<char> {
        let mut ret = Tilemap::new_empty();
        for row in std::io::stdin().lines() {
            let row = row.unwrap();
            ret.add_row(row.chars());
        }
        ret
    }
}