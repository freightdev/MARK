# 🧠 MARK — Memory Aware Routing Kernel

![License: PMSL-v1](https://img.shields.io/badge/license-PMSL--v1-brightgreen?style=flat-square)

> MARK is a markdown-native memory routing system for humans, agents, and hybrid tools.
> It reads `.md(x)`, `.mark`, and `.mstp` files and turns them into structured, usable memory flows.

---

## ⚠️ Still in Development

These project is **actively being built**, refined, and soon to be stress-tested in the real world.

* Features may change as I break, test, and rebuild from the ground up
* Some logic is experimental or placeholder until deployment-ready
* Nothing gets released until it’s strong enough to be trusted by the people it’s built for

If you’re reading this, you’re early.
If you’re using this, you’re part of the build.

> *“I won’t ship anything I wouldn’t use for myself.”* — Jesse

![status](https://img.shields.io/badge/status-in%20development-orange?style=flat-square)

---

## 📆 What It Does

* Parses markdown-based memory files
* Detects and indexes **marks**, **pages**, **ribbons**, and **trails**
* Structures `.mstp` (Markdown Storytelling Protocol) logic into flows

---

## 🤪 Example Usage

```bash
mark init            # creates `mark.mstp` --setup
mark index           # creates `index.mark`
mark law             # loads `law.mark`
mark store           # loads `store.mark`
mark shelf           # loads `shelf.mark`
mark case            # loads `case.mark`
mark stem            # loads `stem.mark`
mark book            # loads `book.mark`
mark cover           # loads `cover.mark`
mark cover front     # loads `front_cover.mark`
mark cover back      # loads `back_cover.mark`
mark cover side      # loads `side_cover.mark`
mark page            # loads `page.mark`
mark page front      # loads `front_page.mark`
mark page back       # loads `back_page.mark`
mark page fold       # loads `fold_page.mark`
mark page tare       # loads `tare_page.mark`
mark mark            # loads `mark.mark`
mark marker          # loads `marker.mark`
mark dust            # loads `dust.mark`
mark ribbon          # loads `ribbon.mark`
mark trail           # loads `trail.mark`
mark ink             # loads `ink.mark`
```

---

## 🪩 Memory Flow Structure

```text
book.mark ─▶ page.mark ─▶ marker.mark ─▶ store.mark
    ▲            ▲             ▲              ▲
    │            |             |              |
 law.mark    stem.mark      ink.mark    {shelf,case}.mark
    ▲            ▲             ▲
    │            |             |
dust.mark    mark.mark     trail.mark
    ▲            ▲
    |            |
cover.mark  ribbon.mark
```

> 💡 Any file in the system may be tracked using the `.trail.*` suffix:
>
> * `packet-process.trail.ribbon`
> * `scan.trail.mark`
> * `fed.trail.book`

---

## 🛠️ Tech Stack

```
Rust • Markdown • CLI-first • Memory Routing • Zero External APIs
```

---

## 📜 License

MARK is licensed under the [**PMSL-v1**](https://github.com/freightdev/PMSL) — **Public Memory Structure License**:

> You can use it. You can modify it. You can build with it.
> But if you improve how memory is structured — you must share it.

---

## ⚖️ Related Licenses

* Bookmark Studio — [MIT License](https://opensource.org/licenses/MIT)
* Bookstore - Proprietary (royalty-based)
* Marker Ink — Proprietary (royalty-based)

---

## ✅ What You Can Do

* ✔️ Use MARK in personal, commercial, or open-source projects
* ✔️ Embed `.mark`, `.trail`, and `.mstp` logic into any agent or platform
* ✔️ Build tools, workflows, dashboards, or full agent OS layers
* ✔️ Sell or distribute tools built with MARK, with attribution to PMSL

---

## ❌ What You Can’t Do

* ❌ Rename MARK and claim originality
* ❌ Lock `.mstp` or routing logic behind closed license without contribution
* ❌ Claim MARK, Bookmark Studio, or Bookstore unmodified as your own
* ❌ Obfuscate or hide the origin of routing logic derived from MARK

---

## ✍️ Author

**Jesse Edward Eugene Wayne Conley**
* 🧠 [github.com/freightdev](https://github.com/freightdev)
* 💬 [jesse.freightdev@gmail.com](mailto:jesse.freightdev@gmail.com)

> "If the memory is worth keeping, the mark should hold weight."

---

## 💛 Support

If this project helps you or inspires your agent builds:
👉 [Buy Me a Coffee](https://coff.ee/freightdev)

> Every dollar supports building tools for the ones still behind the wheel.
