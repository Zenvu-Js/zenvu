//! `Zenvu create` â€” Scaffold a new Zenvu.js project.

use anyhow::Result;
use std::fs;
use std::path::Path;

pub async fn run(name: &str, template: &str, typescript: bool) -> Result<()> {
    println!("\n  ðŸ”µ Zenvu.js â€” Creating project: {}\n", name);

    let project_dir = Path::new(name);
    if project_dir.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }

    // Supported Starter Kits
    println!("  ðŸ“¦ Fetching Template Starter Kit: {}", template);
    match template {
        "admin-dashboard" => println!("  (Using Admin Dashboard Starter Kit)"),
        "ecommerce" => println!("  (Using E-Commerce Starter Kit)"),
        "blog" => println!("  (Using Static Blog Starter Kit)"),
        _ => println!("  (Using Default Empty Starter Kit)"),
    }

    // Create directory structure
    let dirs = [
        "", "src", "src/components", "src/pages", "src/stores",
        "src/assets", "public",
    ];
    for dir in &dirs {
        fs::create_dir_all(project_dir.join(dir))?;
    }

    let ext = if typescript { "ts" } else { "js" };

    // Generate package.json
    let package_json = format!(r#"{{
  "name": "{}",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {{
    "dev": "Zenvu dev",
    "build": "Zenvu build",
    "preview": "Zenvu preview"
  }},
  "dependencies": {{
    "@zenvu/runtime": "^0.1.0"
  }},
  "devDependencies": {{
    "@zenvu/cli": "^0.1.0"{}
  }}
}}"#, name, if typescript { ",\n    \"typescript\": \"^5.7.0\"" } else { "" });
    fs::write(project_dir.join("package.json"), package_json)?;

    // Generate Zenvu.config
    let config = format!(r#"import {{ defineConfig }} from '@zenvu/cli';

export default defineConfig({{
  mode: '{}',
  typescript: {},
  plugins: [],
  server: {{
    port: 3000,
    open: true,
  }},
  build: {{
    target: 'es2022',
    minify: true,
    sourcemap: true,
  }},
}});
"#, template, typescript);
    fs::write(project_dir.join(format!("Zenvu.config.{}", ext)), config)?;

    // Generate main entry
    let main_ts = r#"import App from './App.Zenvu';

const app = App(document.getElementById('app')!);

// Enable HMR in development
if (import.meta.hot) {
  import.meta.hot.accept();
}
"#;
    fs::write(project_dir.join(format!("src/main.{}", ext)), main_ts)?;

    // Generate App.Zenvu
    let app_zenvu = r#"<script lang="ts">
  import Header from './components/Header.Zenvu';

  let title = 'Welcome to Zenvu.js';
  let count = 0;

  function increment() {
    count += 1;
  }
</script>

<template>
  <div class="app">
    <Header :title="title" />
    <main>
      <h1>{{ title }}</h1>
      <p class="tagline">Write like Vue. Compile like Rust. Run like nothing's there.</p>
      <div class="counter-card">
        <button @click="increment">Count: {{ count }}</button>
      </div>
    </main>
  </div>
</template>

<style scoped>
  .app {
    max-width: 1280px;
    margin: 0 auto;
    padding: 2rem;
    text-align: center;
    font-family: 'Inter', system-ui, sans-serif;
  }

  h1 {
    font-size: 3rem;
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    margin-bottom: 0.5rem;
  }

  .tagline {
    color: #94a3b8;
    font-size: 1.125rem;
    margin-bottom: 2rem;
  }

  .counter-card {
    padding: 2rem;
  }

  button {
    padding: 0.75rem 2rem;
    font-size: 1rem;
    font-weight: 600;
    color: white;
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    border: none;
    border-radius: 0.75rem;
    cursor: pointer;
    transition: transform 0.15s, box-shadow 0.15s;
  }

  button:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(59, 130, 246, 0.35);
  }

  button:active {
    transform: translateY(0);
  }
</style>
"#;
    fs::write(project_dir.join("src/App.Zenvu"), app_zenvu)?;

    // Generate Header component
    let header_zenvu = r#"<script lang="ts">
  export let title: string = 'Zenvu.js';
</script>

<template>
  <header class="header">
    <div class="logo">ðŸ”µ</div>
    <nav>
      <a href="/">Home</a>
      <a href="/docs">Docs</a>
      <a href="/examples">Examples</a>
    </nav>
  </header>
</template>

<style scoped>
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 0;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .logo { font-size: 2rem; }

  nav { display: flex; gap: 1.5rem; }

  nav a {
    color: #94a3b8;
    text-decoration: none;
    font-weight: 500;
    transition: color 0.2s;
  }

  nav a:hover { color: #3b82f6; }
</style>
"#;
    fs::write(project_dir.join("src/components/Header.Zenvu"), header_zenvu)?;

    // Generate index.html
    let index_html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <meta name="description" content="A Zenvu.js application" />
  <title>{}</title>
  <link rel="icon" type="image/x-icon" href="/favicon.ico" />
</head>
<body>
  <div id="app"></div>
  <script type="module" src="/src/main.{}"></script>
</body>
</html>"#, name, ext);
    fs::write(project_dir.join("index.html"), index_html)?;

    // tsconfig.json
    if typescript {
        let tsconfig = r#"{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "strict": true,
    "jsx": "preserve",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "esModuleInterop": true,
    "lib": ["ES2022", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,
    "noEmit": true,
    "types": ["@zenvu/runtime/types"]
  },
  "include": ["src/**/*.ts", "src/**/*.Zenvu"]
}"#;
        fs::write(project_dir.join("tsconfig.json"), tsconfig)?;
    }

    // .gitignore
    fs::write(project_dir.join(".gitignore"), "node_modules\ndist\n.Zenvu\n*.local\n")?;

    println!("  âœ… Project created successfully!\n");
    println!("  Next steps:");
    println!("    cd {}", name);
    println!("    npm install");
    println!("    Zenvu dev\n");

    Ok(())
}
