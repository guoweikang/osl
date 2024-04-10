# OS Layer
This is an OS interface abstraction layer developed for cross-kernel drivers.

# How to use

## Linux
**Developers should have the ability to compile R4L for Linux**

Currently, RUST development under Linux is maintained by the R4L community,
and drivers developed following the cross-kernel driver framework will not
directly call R4L's kernel module. 

Unfortunately, Linux currently does not support Cargo to build external modules.
Considering the particularity of OSL (it needs to encapsulate the Linux kernel),
decided to use OSL as the upper-layer module of the kernel and compile it into the
RUST universal library of Linux. The following are specific instructions.

Here is a demonstration of how to apply OSL on Linux.

```shell
cd YOUR_LINUX_KERNEL_DIR/rust
git clone https://github.com/guoweikang/osl.git
git am osl/patches/001-Rust-support-OSL-crate-lib.patch
```

compile Linux for R4L, osl can be used like `kernel` 
```
use osl::*;
```

# Reference
Here are some ongoing projects on how to develop cross-kernel drivers using OSL
 - I2C:




