A small implementation of hot reloading in Rust.

The basic premise is to split the application into two parts: a shared library
and a thin executable wrapper.

Code that is to be hot reloaded is put into the shared library, and then
functions in that library are called from the executable.

When pressing F5, the code is rebuilt via `cargo` and the resulting shared
library is then re-loaded.

To get around lock files on Windows, the shared library is first copied before
it's loaded.
