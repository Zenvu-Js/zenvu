//! `Zenvu generate` â€” Scaffold components, pages, stores, layouts.
use anyhow::Result;
use std::fs;
use std::path::Path;

pub async fn run(gen_type: &str, name: &str) -> Result<()> {
    match gen_type {
        "component" | "c" => generate_component(name)?,
        "page" | "p" => generate_page(name)?,
        "store" | "s" => generate_store(name)?,
        "layout" | "l" => generate_layout(name)?,
        _ => anyhow::bail!("Unknown type: {}. Use: component, page, store, layout", gen_type),
    }
    Ok(())
}

fn generate_component(name: &str) -> Result<()> {
    let path = Path::new("src/components").join(format!("{}.Zenvu", name));
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, format!(r#"<script lang="ts">
  // {name} component
</script>

<template>
  <div class="{lower}">
    <slot />
  </div>
</template>

<style scoped>
  .{lower} {{}}
</style>
"#, name = name, lower = name.to_lowercase()))?;
    println!("  âœ… Created component: {}", path.display());
    Ok(())
}

fn generate_page(name: &str) -> Result<()> {
    let path = Path::new("src/pages").join(format!("{}.Zenvu", name));
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, format!(r#"<script lang="ts">
  import {{ useRoute }} from '@zenvu/router';
  const route = useRoute();
  let title = '{name}';
</script>

<template>
  <main class="page-{lower}">
    <h1>{{{{ title }}}}</h1>
  </main>
</template>

<style scoped>
  .page-{lower} {{ padding: 2rem; }}
</style>
"#, name = name, lower = name.to_lowercase()))?;
    println!("  âœ… Created page: {}", path.display());
    Ok(())
}

fn generate_store(name: &str) -> Result<()> {
    let path = Path::new("src/stores").join(format!("{}.ts", name.to_lowercase()));
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, format!(r#"import {{ defineStore }} from '@zenvu/store';

export const use{name}Store = defineStore('{lower}', {{
  state: () => ({{
    // Define your state here
  }}),

  actions: {{
    // Define your actions here
  }},

  getters: {{
    // Define your getters here
  }},
}});
"#, name = name, lower = name.to_lowercase()))?;
    println!("  âœ… Created store: {}", path.display());
    Ok(())
}

fn generate_layout(name: &str) -> Result<()> {
    let path = Path::new("src/layouts").join(format!("{}.Zenvu", name));
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, format!(r#"<script lang="ts">
  // {name} layout
</script>

<template>
  <div class="layout-{lower}">
    <slot name="header" />
    <main><slot /></main>
    <slot name="footer" />
  </div>
</template>

<style scoped>
  .layout-{lower} {{
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }}
  main {{ flex: 1; }}
</style>
"#, name = name, lower = name.to_lowercase()))?;
    println!("  âœ… Created layout: {}", path.display());
    Ok(())
}
