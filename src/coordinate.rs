//! 9D Phext Coordinate System
//! 
//! Coordinates are 9 dimensions, each 1-9 (mod 9+1 arithmetic)

use std::fmt;

/// 9D coordinate in phext space
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coordinate {
    pub dims: [u8; 9],
}

impl Coordinate {
    pub fn new(dims: [u8; 9]) -> Self {
        Self { dims }
    }

    pub fn origin() -> Self {
        Self { dims: [1; 9] }
    }

    /// Random coordinate within extents
    pub fn random(extents: &[u8; 9]) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut dims = [0u8; 9];
        for i in 0..9 {
            dims[i] = rng.gen_range(1..=extents[i]);
        }
        Self { dims }
    }

    /// Get neighbor in direction along dimension
    /// Returns None if would go out of bounds (1-extent)
    pub fn neighbor(&self, dim: usize, delta: i8, extents: &[u8; 9]) -> Option<Self> {
        if dim >= 9 {
            return None;
        }
        let new_val = self.dims[dim] as i16 + delta as i16;
        if new_val < 1 || new_val > extents[dim] as i16 {
            return None;
        }
        let mut new_coord = *self;
        new_coord.dims[dim] = new_val as u8;
        Some(new_coord)
    }

    /// Get all Von Neumann neighbors (±1 in each dimension)
    /// Up to 18 neighbors in 9D
    pub fn von_neumann_neighbors(&self, extents: &[u8; 9]) -> Vec<Self> {
        let mut neighbors = Vec::with_capacity(18);
        for dim in 0..9 {
            if let Some(n) = self.neighbor(dim, -1, extents) {
                neighbors.push(n);
            }
            if let Some(n) = self.neighbor(dim, 1, extents) {
                neighbors.push(n);
            }
        }
        neighbors
    }

    /// Manhattan distance in 9D
    pub fn distance(&self, other: &Self) -> u32 {
        self.dims.iter()
            .zip(other.dims.iter())
            .map(|(a, b)| (*a as i16 - *b as i16).unsigned_abs() as u32)
            .sum()
    }

    /// Convert to linear index (for visualization)
    pub fn to_index(&self, extents: &[u8; 9]) -> usize {
        let mut idx = 0usize;
        let mut multiplier = 1usize;
        for i in 0..9 {
            idx += (self.dims[i] as usize - 1) * multiplier;
            multiplier *= extents[i] as usize;
        }
        idx
    }

    /// Convert from linear index
    pub fn from_index(mut idx: usize, extents: &[u8; 9]) -> Self {
        let mut dims = [1u8; 9];
        for i in 0..9 {
            dims[i] = (idx % extents[i] as usize) as u8 + 1;
            idx /= extents[i] as usize;
        }
        Self { dims }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}/{}.{}.{}/{}.{}.{}",
            self.dims[0], self.dims[1], self.dims[2],
            self.dims[3], self.dims[4], self.dims[5],
            self.dims[6], self.dims[7], self.dims[8])
    }
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coord({})", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin() {
        let c = Coordinate::origin();
        assert_eq!(c.dims, [1, 1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_neighbor() {
        let extents = [3u8; 9];
        let c = Coordinate::new([2, 2, 2, 2, 2, 2, 2, 2, 2]);
        
        let n = c.neighbor(0, 1, &extents).unwrap();
        assert_eq!(n.dims[0], 3);
        
        let n = c.neighbor(0, -1, &extents).unwrap();
        assert_eq!(n.dims[0], 1);
    }

    #[test]
    fn test_von_neumann() {
        let extents = [3u8; 9];
        let c = Coordinate::new([2, 2, 2, 2, 2, 2, 2, 2, 2]);
        let neighbors = c.von_neumann_neighbors(&extents);
        assert_eq!(neighbors.len(), 18); // 2 per dimension × 9 dimensions
    }

    #[test]
    fn test_index_roundtrip() {
        let extents = [3u8; 9];
        let c = Coordinate::new([2, 3, 1, 2, 3, 1, 2, 3, 1]);
        let idx = c.to_index(&extents);
        let c2 = Coordinate::from_index(idx, &extents);
        assert_eq!(c, c2);
    }
}
