# Week 1 (08.02.2024)
Remember that signed means pos/neg, and that floats are always signed.

Consts need to have a type, and cant use an expression as its value, unless its a certain type of expression.

Shadowing is when you assign a new value to an unmut variable. The data type doesnt matter when shadowing, but does when the variable is mut.

Rust supports tuples and arrays. () is a unit type, or rather an empty tuple.


There are 3 loop types. Loop, While and For. Each has its own use case.

Keep in mind that statements do not return anything, where as expressions do.

The stack and heap are important - stack hold everything that has a knowable size, including pointers to data that is stored on the Heap. Rust also handles when two value are assigned the same Heap held value, by simply invalidating the first variable (dropping it), since the value has been "moved".

# Week 1 (06.02.2024)

Worked on the chapter 2 random number generator.

Take note of `cargo doc --open`, will come in handy.

Learned about the basic of Rust - how to use the match statement, and how to sort
of employ an enum in the way of `cmp`. Learned a way to inference a data type as a
different one, and used a loop.
