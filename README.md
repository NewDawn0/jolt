# ⚡ Jolt-Notes

**A note-taking app that respects your CPU, your RAM, and your intelligence.**
**Born of spite, benchmarks, and an unflinching refusal to tolerate web-tech mediocrity.**

Jolt is an [Excalidraw](https://excalidraw.com)-inspired canvas note-taking app for those who are done pretending that web bloat is unavoidable.

Written primarily in Rust and compiled to WebAssembly, Jolt keeps JavaScript on a short leash — because performance is not a "nice-to-have." It’s a mandate.

---

## Features

* [ ] Freeform canvas note-taking
* [ ] Shapes, text, and diagramming
* [ ] Massive documents without exponential slowdown
* [ ] Deterministic, inspectable data model

Designed for longevity, not for trend-chasing.

---

## The Problem

Modern note-taking apps have accomplished something truly remarkable:

They manage to **get slower** the more you use them. **Exponentially slower.**

This is not an accident. It is the **logical outcome of moral laziness**:

* Building long-lived applications on frameworks designed for form validation
* Wrapping abstraction upon abstraction, calling it "maintainable" while your computer dies a little inside
* Optimizing for developer comfort and billing the user in CPU cycles

If your app stutters, hoovers RAM like a vacuum cleaner, or requires a small data center just to open a note… the problem is **not you**. The problem is **your tech stack.**

---

## Performance

Modern note-taking apps get slower as you take more notes. **Exponentially slower.**

This is not a mystery. It is the feature of your technology choices:

* Building long-lived, state-heavy apps on frameworks designed for form validation
* Layering abstraction upon abstraction and calling it "maintainable"
* Optimizing for developer convenience and billing the user in CPU cycles

---

## Jolt Exists Because

* Notes apps should **not decay over time**
* Idle RAM should **not approach a gigabyte**
* "It’s just a web app" is **never** an excuse

If your current notes app:

* Slows down as you add more notes
* Gobbles RAM like a junkie
* Stutters because "state updates are expensive"

…then congratulations. You are using the **wrong technology.**

Jolt is the antidote. **Performance is not optional. It is non-negotiable.**

---

### Rendering: GPU or Nothing

Jolt renders primarily on the GPU via [WebGL2](https://www.khronos.org/webgl), because:

* GPUs are extremely good at drawing things
* CPUs are extremely bad at pretending to be GPUs
* Redrawing everything every frame in JavaScript is not "simple." It’s negligent

> If your rendering strategy is "just clear the canvas and redraw" — you are **wasting electricity, memory, and human dignity.**

WebGL2 is not a suggestion. It is **the battlefield where performance is won.**

---

## Privacy (A radical concept in 2026)

* No telemetry
* No analytics
* No tracking
* No data sales
* No "AI insights" training on your precious notes

Your notes live **solely** on your machine.

If you don’t trust us:

* Read the source code
* Compile it yourself
* Verify it

I am not asking for trust. I am providing evidence.
Your data is **not interesting to us.** Digital privacy matters now more than ever — especially in the era of "1984" legislation.

---

## About AI features (or lack thereof)

Jolt does **not**:

* Spoon-feed you AI slop disguised as "productivity" (looking at you, Microslop)
* Upload your notes for "processing" to some corporate black box
* Pretend a transformer model makes you smarter

Yes. You are allowed to think.

---

## Contributing

FFMPEG said it best:

> Talk is cheap. Send patches.

Hot takes are free. Code is the currency.

If your contribution:

* Adds unnecessary dependencies
* Sacrifices performance for convenience

…it will be **rejected mercilessly.**

Before contributing, read `doc/FAQ.md`. It explains why.

---

### Design Philosophy

* 🦀 **Rust First**: Enforces invariants. No hope, no prayers.
* ⚙️ **WASM as Infrastructure**: Not a demo. Not a toy. The **engine.**
* ✂️ **Minimal JavaScript**: It coordinates. It does **not decide.**
* 📦 **Dependencies Are a Cost**: Every one must justify existence.
* 🔥 **Performance Is a Feature**: Regressions are sins. Excuses are heresy.

---

## License: AGPL-3.0-or-later

This is **not a suggestion.** It is **law.**

Take, modify, deploy, or offer the code in any form? You **must** share the source.

* No excuses
* No loopholes
* No "we didn’t know"
* No SaaS escape hatches

---

### About Freeriding Companies

Let’s be crystal clear. If your business model is:

* "It’s open source, so it’s free."
* "Nobody will notice."
* "We’ll deal with it when it becomes a problem."

You are **not clever.** You are a violator. You are **part of the problem.**

Yes — AGPL-3 is here **to make violation maximally painful.** Suffer it, and learn.

---

## Project Status

* Under **heavy development**
* APIs may change without mercy
* Features may appear… or vanish

Until Jolt reaches stability, continue using your current note-taking app. Personally, I recommend [Excalidraw](https://excalidraw.com) — the spark that ignited this rebellion.
