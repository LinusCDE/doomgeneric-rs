# doomgeneric-rust

This was primarily a test whether a doom port could be integrated into rust without much hassle (to later integrate with another platform).

The awesome doom port [doomgeneric](https://github.com/ozkl/doomgeneric) was used as a basis.

This repo was originally started as [piston-doom](https://github.com/LinusCDE/piston-doom) and later turned into a standalone lib. Therefore this is technically a fork of that repo with the piston-components removed.

![piston-doom](https://transfer.cosmos-ink.net/12oclH/Unbenannt.png)

---

This lib uses [doomgeneric](https://github.com/ozkl/doomgeneric) as a submodule. Therefore ensure this gets cloned as well or the build will fail.

For more instructions and a running demo, see [piston-doom](https://github.com/LinusCDE/piston-doom).

---

TODO:

- Setting `myargv` and `myargc` to support the `-iwad`-flag: You need to the file named in a default way for now. The console will show you all the names
