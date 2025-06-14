# 🧠 MARK CLI — Memory Assisted Routing Kernel

![License: GMPL-v1](https://img.shields.io/badge/license-GMPL--v1-brightgreen?style=flat-square)

> MARK is a markdown-native memory routing system for humans, agents, and hybrid tools.
> It reads `.md`, `.mark`, `.marker`, `.bookmark`, and `.mstp` files and turns them into structured, usable memory flows.

---

## 📦 What It Does

* Parses any markdown-based memory
* Detects and indexes marks and bookmarks
* Structures `.mstp` (Markdown Storytelling Protocol) files
* Creates runnable `book.mark` files for agent execution
* Powers agents like PacketPilot, WhisperWitness, and CargoConnect

---

## 🧪 Example Usage

```bash
mark init
mark book covers
mark book marks
mark route
```

Creates:

* `.mark/cover.mark`
* `.mark/mark.mark`
* `mark.mstp`
* `book.mark`

---

## 📁 Folder Layout

```
project/
├── README.md
├── GMPL-v1.md
├── .mark/
│   ├── cover.mark
│   ├── mark.mark
│   └── book.mark
├── src/
│   └── ...
```

---

## 🛠 Tech Stack

```
Rust • Markdown • CLI-first • Memory Routing • Zero External APIs
```

---

## 📄 License

MARK CLI is licensed under the **[GMPL-v1](./GMPL-v1.md)** — General Memory Public License:

> You can use, modify, and build with this.
> But if you improve how memory is structured, you must share it.

> BookOS and Bookmark Studio are MIT.
> MARK is GMPL.

---

## ✍️ Author

**Jesse Edward Eugene Wayne Conley**
📬 [jesse.freightdev@gmail.com](mailto:jesse.freightdev@gmail.com)
🔗 [github.com/freightdev](https://github.com/freightdev)

> "I didn’t build this to automate the road. I built it so no one gets left behind."

---

## 💛 Support

If this project helps you or inspires your agent builds:
[Buy Me a Coffee](https://coff.ee/freightdev)

Every dollar goes toward tools for the ones still behind the wheel.

☕️ Jesse — [freightdev](https://github.com/freightdev)
