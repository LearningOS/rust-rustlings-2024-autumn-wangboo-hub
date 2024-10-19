mod foo {
    pub fn my_demo_function(a: u32) -> u32 {
        a + 1
    }

    pub fn my_demo_function_alias(a: u32) -> u32 {
        my_demo_function(a)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_functions() {
            // The externally imported functions are UNSAFE by default
            // because of untrusted source of other languages. You may
            // wrap them in safe Rust APIs to ease the burden of callers.
            //
            // SAFETY: We know those functions are aliases of a safe
            // Rust function.
            unsafe {
                my_demo_function(123);
                my_demo_function_alias(456);
            }
        }
    }
}