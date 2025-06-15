# 🧠 MARK CLI — Memory Aware Routing Kernel

![License: PMSL-v1](https://img.shields.io/badge/license-PMSL--v1-brightgreen?style=flat-square)

> MARK is a markdown-native memory routing system for humans, agents, and hybrid tools.
> It reads `.md(x)`, `.mark`, `.marker`, `.book`, `.page`, `.ribbon`, `.trail`, and `.mstp` files and turns them into structured, usable memory flows.

---

## ⚠️ Still in Development

These projects are **actively being built**, refined, and soon to be stress-tested in the real world.

* Features may change as I break, test, and rebuild everything from the ground up
* Some logic is experimental or placeholder until it's solid enough for real deployment
* Nothing gets released until it’s strong enough to be trusted by the people it’s built for

If you’re reading this, you’re early.
If you’re using this, you’re part of the build.

> *“I won’t ship anything I wouldn’t use for myself.”* — Jesse

![status](https://img.shields.io/badge/status-in%20development-orange?style=flat-square)

---

## 📦 What It Does

* Parses any markdown-based memory
* Detects and indexes marks and bookmarks
* Structures `.mstp` (Markdown Storytelling Protocol) files
* Creates runnable `book.mark` files for execution

---

## 🤪 Example Usage

```bash
mark init  # `mark.mstp` --setup
mark book  # `book.mark` --setup
mark cover  # `cover.mark` --setup
mark mark  # 'mark.mark' --setup
```

---

## 📁 Folder Layout

```
markcli/
├── README.md
├── GMPL-v1.md
├── .mark/
│   ├── book.mark
│   ├── cover.mark
│   ├── mark.mark
│   └── mark.mstp
└── src/
    └── ...
```

---

## 🛠️ Tech Stack

```
Rust • Markdown • CLI-first • Memory Routing • Zero External APIs
```

---

## 📿 License

MARK CLI is licensed under the [**GMPL-v1**](./GMPL-v1.md) — **General Memory Public License**:

> You can use it. You can modify it. You can build with it.
> But if you improve how memory is structured — you must share it.

---

## ⚖️ Related Licenses

* Bookmark: [MIT License](https://opensource.org/licenses/MIT)

---

## ✅ What You Can Do

* ✔️ Use MARK CLI in personal, commercial, or open-source projects
* ✔️ Embed `.mark` and `.mstp` logic into any platform
* ✔️ Build agents, tools, UIs, APIs, and full memory kernels using MARK's structure
* ✔️ Create commercial systems powered by MARK as long as attribution is preserved

---

## ❌ What You Can’t Do

* ❌ Rename `MARK` and claim originality
* ❌ Close-source improvements to `.mstp` or memory routing based on MARK
* ❌ Claim unmodified parts of BookOS, Bookshelf, or MARK CLI as your own
* ❌ Obscure what version of MARK you're using

---

## ✍️ Author

**Jesse Edward Eugene Wayne Conley**

* 📬 [jesse.freightdev@gmail.com](mailto:jesse.freightdev@gmail.com)
* 🔗 [github.com/freightdev](https://github.com/freightdev)

> "I didn’t build this to automate the road. I built it so no one gets left behind."

---

## 💛 Support

If this project helps you or inspires your agent builds:
[Buy Me a Coffee](https://coff.ee/freightdev)

Every dollar goes toward tools for the ones still behind the wheel.

️ Jesse — [freightdev](https://github.com/freightdev)
