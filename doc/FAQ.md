# Frequently Asked Questions

*(or: A Preemptive Response to Questions We’re Exhausted by)*

---

## Why not just use React / Vue / Svelte / $FRAMEWORK?

Because **Jolt is an application**, not a framework hostage situation.

Frameworks are optimized for:

* **Developer throughput**
* **UI churn**
* **Short-lived state**

Jolt is optimized for:

* **Long-lived documents**
* **Large state graphs**
* **Predictable, unyielding performance**

These objectives are **mutually incompatible**.

---

## Why do you use so much [JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript)?

I do not hate it. I **detest using JavaScript for tasks it was never designed to handle**.

JavaScript is suitable for:

* **Glue code**
* **Event handling**
* **Small snippets of logic**

But it arguably should **never** be used for:

* **Critical performance logic**
* **Applications outside the browser** (resulting in bloated, sluggish desktop apps — glaring at you, [Electron](https://www.electronjs.org/))

---

### But [TypeScript](https://www.typescriptlang.org/) fixes that, right?

No.

> **TypeScript is lipstick on a pig.**
> It does **not change the underlying beast.**

It only boosts developer confidence while leaving the pitfalls intact. You still get:

* The same execution model
* The same garbage collector
* The same performance cliffs
* The same footguns — now annotated

---

## Why not Electron?

Shipping a **browser engine with every app** is **not “cross-platform”**.

It is:

* **Redundant**
* **Wasteful**
* **Lazy**

If your desktop app consumes gigabytes of RAM and stutters anyway, the problem is **not optimization**.
The problem is that you shipped a **web browser and called it a product**.

---

## Why are you so hostile to dependencies?

Because dependencies are code you did **not write**, do **not fully understand**, and must now **trust indefinitely**.

The web ecosystem normalized importing half of NPM for:

* Checking if a number is even
* Padding a string
* Doing tasks a for-loop could accomplish

Packages like `isEven` are **not jokes**. They are **symptoms**.

If your app breaks because a maintainer unpublished a joke, that is not bad luck.
That is **negligence**.

---

## But reusing libraries saves time!

So does writing **simple, explicit code** instead of abstracting everything into oblivion.

Jolt prefers:

* **Boring code**
* **Explicit logic**
* **Fewer moving parts**

Time saved today by over-abstraction is **paid back with interest tomorrow**.

---

## Why [Rust](https://www.rust-lang.org/) and [WASM](https://webassembly.org/)?

Because:

* **Explicit memory** is good
* **Predictable performance** is good
* **Crashes are preferable to silent corruption**
* The **compiler is non-optional**

Rust allows us to model invariants that the runtime actually enforces,
while invalid state is nearly inevitable in JavaScript.

WASM allows that logic to run **without begging JavaScript for permission**.

---

## Is this elitist?

No.

This is **defensive programming** against an ecosystem that normalized bloat.

---

## What if I want to add $LIBRARY?

Then explain:

* **What problem it solves**
* **Why it cannot be implemented simply**
* **Why the cost is justified**
* “Everyone uses it” is **not an argument**

---

## Who is this project for?

People who:

* **Care about performance**
* **Care about architecture**
* Are tired of watching their tools **slow down over time**

---

## Who is it not for?

People who think:

* **RAM is infinite**
* **Abstractions are free**
* “Just add another dependency” is a solution

---

## Final Answer to Most Questions

If a suggestion:

* Adds **complexity**
* Adds **dependencies**
* Trades **performance for convenience**

The answer is **probably no**.

And we are **at peace with that**.
