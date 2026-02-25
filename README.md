<a name="readme-top"></a>

<div align="center">

<!-- PROJECT SHIELDS -->

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<br />

<!-- PROJECT LOGO -->
<h1>Nanote</h1>

<p align="center">
	<a href="https://github.com/ninjin-sirisiri/nanote">
    <img src="public/logo.png" alt="Logo" width="80" height="80">
  </a>
  <strong>A blazing-fast, featherweight note-taking app that makes you <em>want</em> to write.</strong>
  <br />
  <em>nano + note — infinitely small, infinitely fast.</em>
  <br /><br />
  <a href="https://github.com/ninjin-sirisiri/nanote"><strong>Explore the docs »</strong></a>
  <br /><br />
  <a href="https://github.com/ninjin-sirisiri/nanote">View Demo</a>
  ·
  <a href="https://github.com/ninjin-sirisiri/nanote/issues/new?labels=bug&template=bug-report.md">Report Bug</a>
  ·
  <a href="https://github.com/ninjin-sirisiri/nanote/issues/new?labels=enhancement&template=feature-request.md">Request Feature</a>
</p>

</div>

---

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
        <li><a href="#key-features">Key Features</a></li>
      </ul>
    </li>
    <li><a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#keyboard-shortcuts">Keyboard Shortcuts</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

---

## About The Project

Most note-taking apps fail you at the very first moment — they're **slow to open, heavy on memory, and clunky to navigate**. By the time the app loads, the thought is already gone.

**Nanote** was built to solve exactly that.

- ⚡ **Sub-0.5s cold start** — powered by native GPU rendering via `gpui`
- 🦀 **Rust + SQLite** — every action (search, switch, save) is instant
- 🪶 **< 50 MB idle memory** — lighter than a browser tab

Whether you're a developer or a non-technical user, Nanote gets out of your way and lets you focus on what matters: **writing**.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

[![Rust][rust-shield]][rust-url]
[![gpui][gpui-shield]][gpui-url]
[![SQLite][sqlite-shield]][sqlite-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Key Features

| Feature                     | Description                                                                 |
| --------------------------- | --------------------------------------------------------------------------- |
| **Live Markdown Rendering** | Write Markdown and see it rendered instantly — like Apple Notes, but faster |
| **Tabs**                    | Open unlimited notes simultaneously in a familiar tab interface             |
| **Folder Tree**             | Infinitely nestable folders with drag-and-drop support                      |
| **Tags**                    | Color-coded tags with multi-tag filtering                                   |
| **Full-Text Search**        | SQLite FTS5-powered search with `tag:` and `folder:` query operators        |
| **Auto-Save**               | Automatically saves 2 seconds after you stop typing                         |
| **Dark / Light Mode**       | Switch themes instantly via shortcut or settings                            |
| **VSCode-like Layout**      | Sidebar + tab bar + editor — a layout you already know                      |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

---

## Getting Started

### Prerequisites

- **Rust** (latest stable) — [Install via rustup](https://rustup.rs/)

  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- On **Windows**, the MSVC build tools are required:
  [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

### Installation

1. Clone the repository

   ```sh
   git clone https://github.com/ninjin-sirisiri/nanote.git
   cd nanote
   ```

2. Build the project

   ```sh
   cargo build --release
   ```

3. Run Nanote

   ```sh
   cargo run --release
   ```

   Or run the compiled binary directly:

   ```sh
   ./target/release/nanote        # macOS / Linux
   .\target\release\nanote.exe    # Windows
   ```

> **Note:** The database (`nanote.db`) is created automatically in your OS's standard application data directory on first launch. No configuration required.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

---

## Usage

### App Layout

```
┌─────────────────────────────────────────────────────┐
│  [Tab: Note A]  [Tab: Note B]  [+]                  │
├──────────────┬──────────────────────────────────────┤
│  🔍 Search   │                                      │
│  ──────────  │  # My Note Title                     │
│  📁 Folders  │                                      │
│   📁 Work    │  Start writing in Markdown...        │
│    📄 Mtg.   │                                      │
│    📄 TODO   │  **Bold**, _italic_, - lists and     │
│   📁 Personal│  [links](https://...) render live.   │
│  ──────────  │                                      │
│  🏷️ Tags     │                                      │
│   🔴 Work    │                                      │
│   🔵 Important│                                     │
├──────────────┤                                      │
│  Status Bar  │                                      │
└──────────────┴──────────────────────────────────────┘
```

### Search Syntax

Nanote supports a powerful search syntax directly in the search bar:

| Query                               | Result                                    |
| ----------------------------------- | ----------------------------------------- |
| `meeting notes`                     | Full-text search across all notes         |
| `tag:work`                          | Filter notes by the "work" tag            |
| `folder:personal`                   | Filter notes inside the "personal" folder |
| `tag:important folder:work meeting` | Combine all operators                     |

### Supported Markdown (MVP)

Headings (`#`, `##`, `###`), **bold**, _italic_, unordered and ordered lists, `- [ ]` checkboxes, `>` blockquotes, `---` horizontal rules, and `[links](url)`.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

---

## Keyboard Shortcuts

| Shortcut                    | Action                                |
| --------------------------- | ------------------------------------- |
| `Ctrl+N`                    | Create a new note                     |
| `Ctrl+W`                    | Close current tab                     |
| `Ctrl+S`                    | Save manually (auto-save also active) |
| `Ctrl+F`                    | Search within current note            |
| `Ctrl+Shift+F`              | Global search (sidebar)               |
| `Ctrl+B` _(outside editor)_ | Toggle sidebar                        |
| `Ctrl+B` _(inside editor)_  | Bold                                  |
| `Ctrl+I`                    | Italic                                |
| `Ctrl+K`                    | Insert link                           |
| `Ctrl+Tab`                  | Next tab                              |
| `Ctrl+Shift+Tab`            | Previous tab                          |
| `Ctrl+,`                    | Open settings                         |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

---

## Roadmap

### ✅ Phase 1 — MVP

- [x] Live Markdown editor
- [x] Folder management (CRUD, nesting, drag-and-drop)
- [x] Tag management (CRUD, colors, filtering)
- [x] Tab interface (unlimited tabs)
- [x] Full-text search with `tag:` / `folder:` operators
- [x] Sidebar toggle
- [x] Dark / Light mode
- [x] Auto-save

### 🔧 Phase 2 — Extended Features

- [ ] Reminder notifications (with background process support)
- [ ] Image & file attachments
- [ ] Note pinning
- [ ] Syntax-highlighted code blocks
- [ ] Export (`.md`, `.zip`) / Import (`.md`)

See the [open issues](https://github.com/ninjin-sirisiri/nanote/issues) for a full list of proposed features and known issues.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

---

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make Nanote better, please fork the repo and create a pull request. You can also open an issue with the `enhancement` label. Don't forget to give the project a star — it means a lot!

1. Fork the project
2. Create your feature branch

   ```sh
   git checkout -b feature/AmazingFeature
   ```

3. Commit your changes

   ```sh
   git commit -m 'Add some AmazingFeature'
   ```

4. Push to the branch

   ```sh
   git push origin feature/AmazingFeature
   ```

5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

---

## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

---

## Contact

Your Name — [@your_twitter](https://twitter.com/ninjin-sirisiri) — email@example.com

Project Link: [https://github.com/ninjin-sirisiri/nanote](https://github.com/ninjin-sirisiri/nanote)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

---

## Acknowledgments

- [gpui](https://github.com/zed-industries/zed/tree/main/crates/gpui) — The GPU-accelerated UI framework powering Nanote's instant rendering
- [SQLite](https://www.sqlite.org/) — The world's most deployed database engine
- [rusqlite](https://github.com/rusqlite/rusqlite) — Ergonomic Rust bindings for SQLite
- [Shields.io](https://shields.io) — Badge generation
- [Best-README-Template](https://github.com/othneildrew/Best-README-Template) — README structure inspiration

<p align="right">(<a href="#readme-top">back to top</a>)</p>

---

<!-- MARKDOWN LINKS & BADGES -->

[contributors-shield]: https://img.shields.io/github/contributors/ninjin-sirisiri/nanote.svg?style=for-the-badge
[contributors-url]: https://github.com/ninjin-sirisiri/nanote/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/ninjin-sirisiri/nanote.svg?style=for-the-badge
[forks-url]: https://github.com/ninjin-sirisiri/nanote/network/members
[stars-shield]: https://img.shields.io/github/stars/ninjin-sirisiri/nanote.svg?style=for-the-badge
[stars-url]: https://github.com/ninjin-sirisiri/nanote/stargazers
[issues-shield]: https://img.shields.io/github/issues/ninjin-sirisiri/nanote.svg?style=for-the-badge
[issues-url]: https://github.com/ninjin-sirisiri/nanote/issues
[license-shield]: https://img.shields.io/github/license/ninjin-sirisiri/nanote.svg?style=for-the-badge
[license-url]: https://github.com/ninjin-sirisiri/nanote/blob/main/LICENSE
[rust-shield]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: https://www.rust-lang.org/
[gpui-shield]: https://img.shields.io/badge/gpui-084CCF?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCI+PC9zdmc+
[gpui-url]: https://github.com/zed-industries/zed/tree/main/crates/gpui
[sqlite-shield]: https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white
[sqlite-url]: https://www.sqlite.org/
