// ------------------------------
// Unsafe : four places where `unsafe` is needed
pub fn run() {
    // ------------------------------
    // Dereference a raw pointer
    {
        let mut num = 5;

        // creating raw pointer is allowed in safe
        // but not dereferencing them
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        let address = 0x012345usize;
        let r = address as *const i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
            // println!("r is: {}", *r); // SEGFAULT
        }
    }
    // ------------------------------
    // Calling an unsafe function or method
    {
        unsafe fn dangerous() {}

        unsafe {
            dangerous();
        }
    }
    // ------------------------------
    // Creating a safe abstraction over unsafe code
    {
        use std::slice;

        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
            }
        }

        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
    // ------------------------------
    // Extern and call external code
    {
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }
}
