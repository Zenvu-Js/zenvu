<div align="center">
  <h1>🟣 Zenvu.js</h1>
  <p><strong>The Web Operating System for the Modern Era.</strong></p>
  <p>Created & Maintained with 💜 by <strong><a href="https://github.com/MuhammadLutfiMuzakiiVY">Muhammad Lutfi Muzakii</a></strong></p>

  <p>
    Zenvu.js is a next-generation full-stack meta-framework built for TypeScript and JavaScript applications, designed to outperform traditional frontend frameworks and modern meta-frameworks in speed, memory efficiency, responsive rendering, and integrated security.
  </p>

  <p>
    <a href="https://zenvujs.dev">Documentation</a> •
    <a href="https://zenvujs.dev/playground">Playground</a> •
    <a href="#quick-start">Quick Start</a>
  </p>
</div>

---

## ⚡ Why Zenvu.js?

While traditional frameworks like React or Vue ship heavy Virtual DOM engines, and modern frameworks like Next.js require complex deployment pipelines, **Zenvu.js** synthesizes the best of all worlds into a single, cohesive, Rust-powered monolith.

### 1. Zero-VDOM Compilation
Zenvu.js compiles your declarative components (using either Vue-like templates or React-like JSX) directly into surgical Vanilla JS DOM updates.
* **$O(1)$ Update Speeds**: We map variables directly to UI TextNodes using a Directed Acyclic Graph (DAG). No Virtual DOM diffing.
* **Micro-Payloads**: The core client-side runtime weighs in at **< 20KB**.

### 2. The Smart Adaptive Engine (Hardware-Aware)
Zenvu.js doesn't just run on browsers; it listens to the hardware:
* **RAM Limiter**: Auto-detects heap size and forces Garbage Collection/Animation Throttling if memory exceeds 500MB (perfect for low-end devices).
* **Battery-Saving Render**: Hooks into the Battery API to instantly suspend 60fps animations when a user's battery drops below 20%.
* **Smart Garbage Collection**: Uses `IntersectionObserver` to automatically pause rendering loops and detach events for components scrolled out of view.

### 3. Integrated Security (The 7 Pillars)
Security is not an afterthought. It is woven into the Rust core:
* **Zenvu Guard**: Edge middleware for Request Protection & DDoS mitigation.
* **Zenvu SafeDOM**: Context-aware anti-XSS rendering engine.
* **Zenvu Shield**: V8 Sandbox isolation for third-party plugins (`zenvu plugin install`).
* **Zenvu Vault**: Built-in AES-GCM 256 state encryption.

### 4. 🌐 The Zenvu Render Engine (Universal Rendering System)
Most frameworks force you to choose between Client-Side Rendering (CSR) or Server-Side Rendering (SSR). Zenvu.js introduces the **Zenvu Render Engine**, a hyper-complex, dynamic rendering pipeline that automatically selects the best rendering strategy for each request.

It natively supports 15 distinct rendering topologies out of the box:
* **The Classics**: Static Site Generation (SSG), Server-Side Rendering (SSR), Client-Side Rendering (CSR).
* **The Modern Edge**: Incremental Static Regeneration (ISR), Edge Rendering (Cloudflare/Vercel natively supported).
* **Advanced Hydration**: Partial Hydration (Zero-JS Islands), Streaming Hydration (Chunked Transfer), Lazy Hydration (Hydrate on scroll/interaction).
* **Next-Gen Paradigms**: Resumable Rendering (Qwik-style serialized state injection), Hybrid Rendering (Mixing SSG Shells with SSR fragments).
* **Hardware-Aware Rendering**:
  * **Smart Render Selection**: Dynamically shifts between SSR and CSR based on the client's network latency (Network Information API).
  * **Device-Aware Rendering**: Analyzes `User-Agent` and physical screen width on the Edge to deliver specialized DOM trees.
  * **Low-End Device Render Fallback**: Disables heavy CSS backdrop filters and switches to flat-rendering on low RAM devices.
  * **Offline Render Cache**: Intercepts render requests and falls back to a locally cached PWA payload if internet connection drops mid-navigation.

---

## 🚀 Quick Start

Get started in seconds with the Rust-powered CLI:

```bash
# Scaffold a new Zenvu.js project (SPA, SSR, or SSG)
npx create-zenvu-app my-app

cd my-app

# Start the dev server with Rust-powered HMR (< 15ms cold start)
zenvu dev
```

---

## 🏗️ The 6 Master Engines of Zenvu.js

Zenvu.js is not a collection of loosely coupled libraries. It is a strictly integrated ecosystem composed of 6 hyper-advanced master engines:

### 1. 🪓 Zenvu Forge Compiler
*The Rust-powered heart of the system.*
* **Zero-Runtime Compilation**: Direct DOM CodeGen bypassing Virtual DOM entirely.
* **Smart Asset Bundling & Auto Chunk Splitting**: Parallel compilation, dead code elimination, and selective incremental rebuilds.
* **WASM Output Support**: Cross-compilation to WebAssembly for edge compute.

### 2. 🌐 Zenvu Render Engine
*The Universal Rendering System.*
* **Continuous Render Spectrum**: Natively supports SSG, SSR, CSR, and ISR.
* **Advanced Hydration**: Streaming Hydration, Partial Hydration (Zero-JS Islands), and Resumable Rendering via Base64 Packed Graphs.
* **The Oracle Pipeline**: Smart Render Selection based on real-time Network Latency and Offline Render Cache failovers.

### 3. 🚀 Zenvu RouterX
*Advanced Edge-First Routing.*
* **Parallel & Intercepting Routes**: Complex layouts handled flawlessly.
* **Middleware & Secure Route Guards**: Edge route execution and realtime route sync.
* **Intelligent Prefetching**: Route transition engine with Fitts's Law predictive prefetching.

### 4. 📱 Zenvu Adaptive Engine
*Responsive-first hardware awareness.*
* **Fluid Layout Engine**: CSS container abstraction and automatic responsive compilation.
* **Hardware Limiters**: RAM-sensitive UI, GPU-aware animations, and Low Battery fallback modes.
* **Dynamic UI Density Mode**: Touch optimization and layout auto-scaling based on viewport.

### 5. 🛡️ Zenvu Secure Core
*Uncompromised Enterprise Security.*
* **V8 Plugin Sandbox**: Strict execution environments and signed dependencies.
* **Anti-XSS Compiler & Anti-CSRF Middleware**: Secure by default hydration channels and automatic CSP generation.
* **Encrypted Internal Store**: Built-in AES-GCM 256 state vaults and runtime anomaly detection.

### 6. 🏢 Zenvu Server Core
*The Built-In Backend Ecosystem.*
* **API Engine & Edge Functions**: Real-time WebSocket server and server middleware.
* **Full-Stack Tooling**: Built-in ORM abstraction, Cron Scheduler, Queue System, and Event Bus.
* **Microservice Bridge**: Cache Manager, Session Manager, and Secure Secrets Vault natively integrated.

---

## 💻 Code Example

Enjoy the freedom of native dual-syntax support (Vue & React styles).

```html
<script setup>
  import { useStore } from '@zenvu/store';
  let count = 0; // Automatically reactive

  function increment() {
    count++;
  }
</script>

<template>
  <div class="counter-card" data-responsive="auto">
    <!-- Auto-updates via surgical pointer binding. No VDOM! -->
    <h1>Count: {{ count }}</h1>
    
    <!-- Motion Engine handles 60fps clicks and touch gestures -->
    <button @click="increment">Increment</button>
  </div>
</template>

<style scoped>
  /* CSS is auto-minified and conditionally loaded */
  .counter-card { padding: 20px; }
</style>
```

---

## 📄 License
Zenvu.js is open-source software licensed under the [MIT License](LICENSE).

<div align="center">
  <sub>Built with 🦀 Rust and 💜 TypeScript by <strong><a href="https://github.com/MuhammadLutfiMuzakiiVY">Muhammad Lutfi Muzakii</a></strong> and the Zenvu.js Core Team.</sub>
</div>
