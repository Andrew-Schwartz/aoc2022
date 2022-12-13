pub const fn array2_like<T, U: Copy, const W: usize, const H: usize>(
    _like: &[[T; W]; H],
    u: U,
) -> [[U; W]; H] {
    [[u; W]; H]
}

#[must_use]
pub const fn parse_number(bytes: &[u8]) -> i16 {
    let mut i = 0;
    let mut n = 0;
    while i < bytes.len() {
        n = n * 10 + (bytes[i] - b'0') as i16;
        i += 1;
    }
    n
}

// taken from stdlib but made const cuz it wasn't for some reason
pub const fn split_array_ref<T, const N: usize>(slice: &[T]) -> (&[T; N], &[T]) {
    let (a, b) = slice.split_at(N);
    // SAFETY: a points to [T; N]? Yes it's [T] of length N (checked by split_at)
    (unsafe { &*a.as_ptr().cast() }, b)
    // unsafe { (&*(a.as_ptr() as *const [T; N]), b) }
}

#[const_trait]
pub trait ByteConstExt {
    fn find(&self, char: u8) -> Option<usize>;
}

impl const ByteConstExt for [u8] {
    fn find(&self, char: u8) -> Option<usize> {
        let mut i = 0;

        while i < self.len() {
            if self[i] == char { return Some(i); }

            i += 1;
        }

        None
    }
}