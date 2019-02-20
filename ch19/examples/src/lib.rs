#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_pointer() {
        let mut n = 8;
        let r1 = &n as *const i32;
        let r2 = &mut n as *mut i32;

        assert_eq!(8, n);
        unsafe {
            assert_eq!(8, *r1);
            assert_eq!(8, *r2);

            *r2 = 5;

            assert_eq!(5, *r1);
            assert_eq!(5, *r2);
        }
        assert_eq!(5, n);
    }

    #[test]
    fn arbitrary_memory_address() {
        let p = 0x012345usize;
        let _r = p as *const i32;
        // not dereferencing that
    }

    #[test]
    fn function_with_unsafe_block() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v;

        let (xs, ys) = split(r, 3);

        assert_eq!(*xs, [1, 2, 3]);
        assert_eq!(*ys, [4, 5, 6]);
    }

}

use std::slice;
fn split(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let p = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(p, mid),
            slice::from_raw_parts_mut(p.offset(mid as isize), len - mid),
        )
    }
}
