#[inline(always)]
pub const fn array2_like<T, U: Copy, const W: usize, const H: usize>(
    _like: &[[T; W]; H],
    u: U,
) -> [[U; W]; H] {
    [[u; W]; H]
}

#[must_use]
#[inline(always)]
pub const fn parse_number(bytes: &[u8]) -> i16 {
    let mut i = 0;
    let mut n = 0;
    while i < bytes.len() {
        n = n * 10 + (bytes[i] - b'0') as i16;
        i += 1;
    }
    n
}

#[const_trait]
pub trait SliceConstExt<T> {
    fn find(&self, item: &T) -> Option<usize> where T: ~const PartialEq;
    fn const_split_array_ref<const N: usize>(&self) -> (&[T; N], &[T]);

    fn take_arr<const N: usize>(self: &mut &Self) -> &[T; N];
    fn take_n(self: &mut &Self, n: usize) -> &[T];
}

impl<T> const SliceConstExt<T> for [T] {
    #[inline(always)]
    fn find(&self, item: &T) -> Option<usize> where T: ~const PartialEq {
        let mut i = 0;

        while i < self.len() {
            if self[i] == *item { return Some(i); }

            i += 1;
        }

        None
    }

    // taken from stdlib but made const cuz it wasn't for some reason
    #[inline(always)]
    fn const_split_array_ref<const N: usize>(&self) -> (&[T; N], &[T]) {
        let (a, b) = self.split_at(N);
        // SAFETY: a points to [T; N]? Yes it's [T] of length N (checked by split_at)
        (unsafe { &*a.as_ptr().cast() }, b)
    }

    #[inline(always)]
    fn take_n(self: &mut &Self, n: usize) -> &[T] {
        let (a, b) = self.split_at(n);
        *self = b;
        a
    }

    #[inline(always)]
    fn take_arr<const N: usize>(self: &mut &Self) -> &[T; N] {
        let (a, b) = self.const_split_array_ref();
        *self = b;
        a
    }
}