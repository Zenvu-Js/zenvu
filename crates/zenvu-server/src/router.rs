//! App Router, File-Based Routing & API Framework
//!
//! Automatically maps the `src/app` or `src/pages` directory into 
//! nested server and client routes, and provides a powerful API routing framework.

use anyhow::Result;
use std::path::Path;
use axum::{
    Router,
    routing::{get, post, put, delete},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Representation of an API Request context
#[derive(Debug, Clone)]
pub struct ApiRequest {
    pub path_params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}

/// Representation of an API Response
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Standardized API router builder
pub struct ApiRouter {
    routes: Router,
}

impl ApiRouter {
    pub fn new() -> Self {
        Self {
            routes: Router::new(),
        }
    }

    /// Register a GET endpoint
    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.routes = self.routes.route(path, get(handler));
        self
    }

    /// Register a POST endpoint
    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.routes = self.routes.route(path, post(handler));
        self
    }

    /// Register a PUT endpoint
    pub fn put<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.routes = self.routes.route(path, put(handler));
        self
    }

    /// Register a DELETE endpoint
    pub fn delete<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.routes = self.routes.route(path, delete(handler));
        self
    }

    /// Consumes the builder and returns the underlying Axum Router
    pub fn into_router(self) -> Router {
        self.routes
    }
}

impl Default for ApiRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Recursively scans the `src/app` or `src/pages` directory and builds the route tree
pub fn generate_file_routes(base_dir: &Path) -> Result<Vec<String>> {
    let mut routes = Vec::new();
    tracing::info!("Scanning {} for File-based routes...", base_dir.display());
    
    // Default mock page routes
    routes.push("/".to_string());
    routes.push("/dashboard".to_string());
    routes.push("/api/users".to_string());

    // Actually scan directory if it exists
    if base_dir.exists() {
        let mut queue = vec![base_dir.to_path_buf()];
        while let Some(current_dir) = queue.pop() {
            if let Ok(entries) = std::fs::read_dir(&current_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        queue.push(path);
                    } else if let Some(ext) = path.extension() {
                        if ext == "zenvu" || ext == "znv" || ext == "ts" || ext == "js" {
                            if let Ok(rel_path) = path.strip_prefix(base_dir) {
                                let mut route_str = rel_path.to_string_lossy().to_string();
                                // Clean up path to route
                                route_str = route_str.replace('\\', "/");
                                if let Some(idx) = route_str.rfind('.') {
                                    route_str = route_str[..idx].to_string();
                                }
                                if route_str.ends_with("/index") {
                                    route_str = route_str[..route_str.len() - 6].to_string();
                                }
                                if route_str.is_empty() {
                                    route_str = "/".to_string();
                                } else if !route_str.starts_with('/') {
                                    route_str = format!("/{}", route_str);
                                }
                                routes.push(route_str);
                            }
                        }
                    }
                }
            }
        }
    }
    
    routes.sort();
    routes.dedup();
    Ok(routes)
}
