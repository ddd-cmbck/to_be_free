# ![TO-BE-FREE](./assets/icon.jpg) To Be Free

<!-- HERO IMAGE -->

# ![TO-BE-FREE](./assets/hero.jpg)

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ddd-cmbck/to_be_free/blob/master/LICENSE)

---

## 📖 Navigation

* [What’s New in v0.1.0](#-whats-new-in-v010)
* [Architecture Overview](#-architecture-overview)
* [Tech Stack](#-tech-stack)
* [How to Run](#-how-to-run)
* [How to Contribute](#-how-to-contribute)
* [License](#-license)

---

## ✨ What’s New in v0.1.0

This release establishes the **core gameplay loop and engine structure**.

### Added in v0.1.0

* Modular Bevy app setup
* Scene initialization and world spawning
* ECS-driven movement system
* Player input handling
* Custom schedules and system ordering
* Component & bundle separation
* Clean module layout for scalability

### Systems Introduced

* **Movement system** (player-controlled motion)
* **Input system** (keyboard-driven interaction)
* **Scene setup systems**
* **Custom schedules** to control execution order

<!-- GAMEPLAY GIF -->
# ![gameplay-gif](./assets/giphy.gif)

## 🧱 Architecture Overview

The project follows **idiomatic Bevy ECS design**:

* **Components** — pure data (movement, transforms, markers)
* **Bundles** — grouped components for spawning entities
* **Systems** — logic operating on queries
* **Schedules** — explicit system ordering and control
* **Modules** — clear separation of responsibilities

This structure allows:

* easy feature expansion
* deterministic system execution
* clean refactoring as mechanics grow

---

## 🛠 Tech Stack

* **Language:** Rust
* **Engine:** Bevy 0.18.0
* **Architecture:** ECS (Entity–Component–System)
* **Scheduling:** Custom Bevy schedules
* **Input:** Bevy input resources
* **Rendering:** Bevy renderer

Designed with future support for:

* procedural environments
* rogue-like progression
* physics-driven flight
* world events & systems

---

## ▶️ How to Run

### Requirements

* Rust `1.92.0`
* Cargo

### Run in development

```bash
cargo run
```

### Run optimized

```bash
cargo run --release
```

## 📄 License

This project is licensed under the **MIT License**.

See the license file here:
👉 [LICENSE](./LICENSE)