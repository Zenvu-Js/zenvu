#!/usr/bin/env node

import fs from 'fs';
import path from 'path';

const projectName = process.argv[2] || 'zenvu-project';
const targetDir = path.resolve(process.cwd(), projectName);

console.log(`\n🟣 \x1b[1m\x1b[35mZenvu.js Project Creator\x1b[0m`);
console.log(`Scaffolding brand new Zenvu.js app in: \x1b[36m${targetDir}\x1b[0m\n`);

if (fs.existsSync(targetDir)) {
  console.error(`\x1b[31mError: Directory '${projectName}' already exists.\x1b[0m`);
  process.exit(1);
}

fs.mkdirSync(targetDir, { recursive: true });
fs.mkdirSync(path.join(targetDir, 'src'), { recursive: true });
fs.mkdirSync(path.join(targetDir, 'src/components'), { recursive: true });

// 1. Write package.json
const packageJson = {
  name: projectName,
  version: "0.1.0",
  private: true,
  type: "module",
  scripts: {
    "dev": "zenvu dev",
    "build": "zenvu build"
  },
  dependencies: {
    "@zenvu/core": "^0.1.0",
    "@zenvu/security": "^0.1.0"
  },
  devDependencies: {
    "typescript": "^5.7.0"
  }
};

fs.writeFileSync(
  path.join(targetDir, 'package.json'),
  JSON.stringify(packageJson, null, 2)
);

// 2. Write tsconfig.json
const tsconfigJson = {
  compilerOptions: {
    target: "ES2022",
    module: "NodeNext",
    moduleResolution: "NodeNext",
    strict: true,
    esModuleInterop: true,
    skipLibCheck: true,
    forceConsistentCasingInFileNames: true
  },
  include: ["src/**/*"]
};

fs.writeFileSync(
  path.join(targetDir, 'tsconfig.json'),
  JSON.stringify(tsconfigJson, null, 2)
);

// 3. Write index.html
const htmlContent = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Zenvu.js App</title>
  <style>
    body {
      background-color: #0b0f19;
      color: #f3f4f6;
      font-family: 'Inter', sans-serif;
      margin: 0;
      display: flex;
      justify-content: center;
      align-items: center;
      height: 100vh;
    }
  </style>
</head>
<body>
  <div id="app"></div>
  <script type="module" src="/src/main.ts"></script>
</body>
</html>
`;

fs.writeFileSync(
  path.join(targetDir, 'index.html'),
  htmlContent
);

// 4. Write src/components/Counter.zenvu
const counterZenvu = `<script setup>
  import { createSignal } from '@zenvu/core';

  // Svelte-like atomical reactivity + solid signals!
  const [count, setCount] = createSignal(0);

  function increment() {
    setCount(c => c + 1);
  }
  function decrement() {
    setCount(c => c - 1);
  }
</script>

<template>
  <div class="counter-card">
    <h1>🟣 Zenvu.js SFC</h1>
    <p class="description">Zero-VDOM high performance reactivity engine</p>
    <div class="count-display">{{ count }}</div>
    <div class="button-group">
      <button @click="decrement" class="btn">-1</button>
      <button @click="increment" class="btn">+1</button>
    </div>
  </div>
</template>

<style scoped>
  .counter-card {
    background: rgba(255, 255, 255, 0.05);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    padding: 30px;
    border-radius: 16px;
    text-align: center;
    box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);
  }
  h1 {
    margin: 0 0 10px 0;
    color: #a855f7;
  }
  .description {
    color: #9ca3af;
    font-size: 14px;
    margin-bottom: 20px;
  }
  .count-display {
    font-size: 48px;
    font-weight: bold;
    margin-bottom: 20px;
    color: #f3f4f6;
  }
  .button-group {
    display: flex;
    gap: 10px;
    justify-content: center;
  }
  .btn {
    background: #a855f7;
    border: none;
    color: white;
    padding: 10px 20px;
    border-radius: 8px;
    font-size: 16px;
    cursor: pointer;
    transition: background 0.2s;
  }
  .btn:hover {
    background: #c084fc;
  }
</style>
`;

fs.writeFileSync(
  path.join(targetDir, 'src/components/Counter.zenvu'),
  counterZenvu
);

// 5. Write src/main.ts
const mainTs = `import { mountApp, defineComponent } from '@zenvu/core';
import Counter from './components/Counter.zenvu';

// Mount our beautiful Zero-VDOM Zenvu App
mountApp('app', () => Counter());
`;

fs.writeFileSync(
  path.join(targetDir, 'src/main.ts'),
  mainTs
);

console.log(`✨ \x1b[32mSuccessfully scaffolded Zenvu.js project!\x1b[0m`);
console.log(`\nRun the following commands to get started:\n`);
console.log(`  \x1b[36mcd ${projectName}\x1b[0m`);
console.log(`  \x1b[36mnpm install\x1b[0m`);
console.log(`  \x1b[36mnpm run dev\x1b[0m\n`);
console.log(`Enjoy building with 🟣 \x1b[1m\x1b[35mZenvu.js\x1b[0m!\n`);
