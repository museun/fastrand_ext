use fastrand::Rng;

fn rand_index(rng: &Rng, bound: usize) -> usize {
    if bound < (u32::MAX as usize) {
        rng.u32(0..bound as u32) as _
    } else {
        rng.usize(0..bound)
    }
}

pub trait IterExt: Iterator + Sized {
    fn choose(mut self, rng: &Rng) -> Option<Self::Item> {
        let (mut lower, mut upper) = self.size_hint();
        let mut pos = 0;
        let mut out = None;

        if Some(lower) == upper {
            if lower == 0 {
                return None;
            }
            return self.nth(rand_index(rng, lower));
        }

        loop {
            if lower > 1 {
                let index = rand_index(rng, lower + pos);
                let skipped = if index < lower {
                    out = self.nth(index);
                    lower - (index + 1)
                } else {
                    lower
                };
                if Some(lower) == upper {
                    return out;
                }
                pos += lower;
                if skipped > 0 {
                    self.nth(skipped - 1);
                }
            } else {
                let next = match self.next() {
                    Some(next) => next,
                    None => return out,
                };
                pos += 1;
                if rng.f64() / pos as f64 > 0.5 {
                    out.replace(next);
                }
            }

            let hint = self.size_hint();
            lower = hint.0;
            upper = hint.1;
        }
    }

    fn choose_multiple_fill(mut self, rng: &Rng, buf: &mut [Self::Item]) -> usize {
        let n = buf.len();
        let mut len = 0;
        while len < n {
            if let Some(e) = self.next() {
                buf[len] = e;
                len += 1;
            } else {
                return len;
            }
        }

        for (i, e) in self.enumerate() {
            let k = rand_index(rng, i + n + 1);
            if let Some(s) = buf.get_mut(k) {
                *s = e;
            }
        }

        len
    }

    fn choose_multiple(mut self, rng: &Rng, count: usize) -> Vec<Self::Item> {
        let mut buf = Vec::with_capacity(count);
        buf.extend(self.by_ref().take(count));

        if buf.len() == count {
            for (i, e) in self.enumerate() {
                let k = rand_index(rng, i + count + 1);
                if let Some(s) = buf.get_mut(k) {
                    *s = e;
                }
            }
        } else {
            buf.shrink_to_fit()
        }
        buf
    }
}

impl<I> IterExt for I where I: Iterator + Sized {}

pub trait SliceExt {
    type Item;

    fn choose(&self, rand: &Rng) -> Option<&Self::Item>;
    fn choose_mut(&mut self, rand: &Rng) -> Option<&mut Self::Item>;
    fn shuffle(&mut self, rand: &Rng);
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn choose(&self, rand: &Rng) -> Option<&Self::Item> {
        self.get(rand.usize(..self.len()))
    }

    fn choose_mut(&mut self, rand: &Rng) -> Option<&mut Self::Item> {
        self.get_mut(rand.usize(..self.len()))
    }

    fn shuffle(&mut self, rand: &Rng) {
        for i in 1..self.len() {
            self.swap(i, rand.usize(..=i))
        }
    }
}
