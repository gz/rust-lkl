[![Build Status](https://travis-ci.org/gz/rust-lkl.svg)](https://travis-ci.org/gz/rust-lkl)

# Linux kernel as a library
Builds the Linux kernel sources as a rust crate for convenient use in your rust project.
Beware of the license since this is essentially Linux.

# Steps the build.rs script does for you
```
git clone --depth=1 https://github.com/lkl/linux.git
make -C tools/lkl
ar rcs liblinux.a tools/lkl/lib/lkl.o
```

To link against it in your application you can add
```
println!("cargo:rustc-link-lib=static=linux");
```
during the build.

# Dependencies & Testing
For running the tests of lkl some python packages are required:

```
pip install junit_xml yamlish
make run-tests
cd tools/lkl
```

# Resources
 * https://github.com/lkl
 * https://lwn.net/Articles/662953/
