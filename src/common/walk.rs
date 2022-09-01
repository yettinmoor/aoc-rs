use std::collections::{HashSet, VecDeque};

use super::matrix::{Coord, Matrix};

pub trait Walk {
    type Item;
    type Index;

    fn walk<F, G, I, St>(
        &self,
        init_coord: Coord,
        initial_st: St,
        walk_fn: F,
        neighbors_fn: G,
    ) -> St
    where
        F: FnMut(&mut St, Option<Self::Index>, Self::Index, Self::Item),
        G: Fn(Self::Index) -> I,
        I: IntoIterator<Item = Self::Index>;
}

impl<M> Walk for M
where
    M: Matrix,
{
    type Item = M::Item;
    type Index = Coord;

    fn walk<F, G, I, St>(
        &self,
        init_coord: Coord,
        mut initial_st: St,
        mut walk_fn: F,
        neighbors_fn: G,
    ) -> St
    where
        F: FnMut(&mut St, Option<Coord>, Coord, M::Item),
        G: Fn(Self::Index) -> I,
        I: IntoIterator<Item = Self::Index>,
    {
        let mut visited = HashSet::<Coord>::with_capacity(self.size());
        let mut queue = VecDeque::new();
        queue.push_back((init_coord, None));
        while let Some((cur, prev)) = queue.pop_front() {
            if !visited.insert(cur) {
                continue;
            }
            walk_fn(&mut initial_st, prev, cur, self.get_coord(cur).unwrap());
            queue.extend(neighbors_fn(cur).into_iter().map(|next| (next, Some(cur))));
        }
        initial_st
    }
}
