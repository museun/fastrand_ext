# fastrand_ext

this crate provides a few extension traits for the wonderful [`fastrand`](https://docs.rs/fastrand/1.4.0/fastrand/) crate.

currently, it replicates some of the iterator/slice stuff from the [`rand`](https://docs.rs/rand/*/rand/) crate

### iterators

- `choose() -> Option<Self::Item>`
- `choose_multiple_fill(&mut [Self::Item]) -> usize`
- `choose_multiple(usize) -> Vec<Self::Item>`

### slices

- `choose() -> Option<&T>`
- `choose_mut() -> Option<&mut T>`
- `shuffle()`

---

License: MIT
