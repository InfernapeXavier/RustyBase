fn main() {
    // Create an empty, mutable vector; no allocation has been made yet, length is 0.
    // Without a type annotation, it needs more type information the first time it's used
    let mut vec = Vec::new();
    // Vector allocates and pushes a single byte, 0, setting the length to 1.
    // The `u8` suffix tells the compiler you're pushing a byte-sized integer and to use this type
    // for all elements of this vector
    vec.push(0u16);
    
    // Insert another 0 byte at index 1, setting the length to 2
    // No suffix necessary because the compiler knows what type to use.
    vec.push("Hello");
    // Can be indexed like a pointer, index is checked at runtime to make sure it's within the existing bounds
    // (you shouldn't worry about the cost of this check, it's cheap and can be optimized out in the right context)
    vec[1] = 255;
    println!("{:#?}", vec);

    
    // Create a brand-new vector with a length of 1024 (1KiB), initialized with all bytes 0
    // The previous vector is no longer accessible, Rust determines here it can deallocate it
    vec = vec![0; 8];
    vec[1] = 127;
    vec[2] = 63;

    // Loop through each byte in the vector, consuming the vector
    // Type of `my_byte` is `byte`
    for my_byte in vec {
        // do_something(my_byte);
        println!("{:#?}", my_byte);
    }
    // `vec` is deallocated at the end of the loop because it is no longer reachable
    // If you want to keep using it afterwards, use `for my_byte in &vec` instead; type of `my_byte` will be `&byte`
    // If you want to mutate the bytes, use `for my_byte in &mut vec`; `my_byte` will be of type `&mut byte`
}