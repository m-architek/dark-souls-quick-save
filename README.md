# Dark Souls Quick Save

Imitates quick save capability in Dark Souls Remastered.

---

**Usage**

- Move application to your save directory (next to `DRAKS0005.sl2`)
- Run application (admin privileges are required)
- Press F5 to copy current save file as quick save file
- Press F6 to restore quick save file as current save file (quick load)

In practice, while in game, if you want to quick save, enter and leave game menu, 
it will force game to save your progress (you will see saving indicator in screen corner).
Then click F5 to quick save (beep confirms that quick save was successful).
If you want to quick load, exit game to main menu, press F6 (double beep is sign of success) and load your game as usual.
You should be back at point when you've done quick save.

App was tested only on Linux.

---

**References**

This app is much simplified version of [EsotericSoftware/dark-souls-saver](https://github.com/EsotericSoftware/dark-souls-saver).

---

**Build from sources**

You need to install [Rust](https://www.rust-lang.org/tools/install) first. Then execute command `cargo build` in application root directory to build application.