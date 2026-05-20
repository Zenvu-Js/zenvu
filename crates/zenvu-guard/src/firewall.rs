//! Request firewall / WAF (Web Application Firewall).
//!
//! Filters incoming requests based on configurable security rules.

/// Firewall rule action.
#[derive(Debug, Clone, PartialEq)]
pub enum Action { Allow, Deny, Log }

/// A firewall rule.
#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub name: String,
    pub pattern: RulePattern,
    pub action: Action,
}

#[derive(Debug, Clone)]
pub enum RulePattern {
    IpRange(String),
    PathPrefix(String),
    HeaderMatch { header: String, value: String },
    BodyContains(String),
    UserAgentMatch(String),
    RequestSizeExceeds(usize),
}

/// Request context for firewall evaluation.
#[derive(Debug)]
pub struct RequestContext {
    pub ip: String,
    pub path: String,
    pub method: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body_size: usize,
    pub user_agent: String,
}

/// Web Application Firewall.
pub struct Firewall {
    rules: Vec<FirewallRule>,
}

impl Firewall {
    pub fn new() -> Self {
        let mut fw = Self { rules: Vec::new() };
        fw.add_default_rules();
        fw
    }

    /// Evaluate a request against all rules.
    pub fn evaluate(&self, ctx: &RequestContext) -> Action {
        for rule in &self.rules {
            if self.matches(&rule.pattern, ctx) {
                tracing::debug!("Firewall rule '{}' matched for {} {}", rule.name, ctx.method, ctx.path);
                if rule.action == Action::Deny {
                    tracing::warn!("🛡️ Request blocked by firewall rule: {}", rule.name);
                    return Action::Deny;
                }
                if rule.action == Action::Log {
                    tracing::info!("📋 Firewall log: rule '{}' triggered", rule.name);
                }
            }
        }
        Action::Allow
    }

    pub fn add_rule(&mut self, rule: FirewallRule) {
        self.rules.push(rule);
    }

    fn matches(&self, pattern: &RulePattern, ctx: &RequestContext) -> bool {
        match pattern {
            RulePattern::PathPrefix(prefix) => ctx.path.starts_with(prefix),
            RulePattern::BodyContains(_s) => false, // Would need body access
            RulePattern::UserAgentMatch(ua) => ctx.user_agent.to_lowercase().contains(&ua.to_lowercase()),
            RulePattern::RequestSizeExceeds(max) => ctx.body_size > *max,
            RulePattern::HeaderMatch { header, value } => {
                ctx.headers.get(header) == Some(value)
            }
            RulePattern::IpRange(range) => ctx.ip.starts_with(range),
        }
    }

    fn add_default_rules(&mut self) {
        // Block overly large requests (default 10MB)
        self.rules.push(FirewallRule {
            name: "max-body-size".into(),
            pattern: RulePattern::RequestSizeExceeds(10_485_760),
            action: Action::Deny,
        });

        // Block known malicious user agents
        self.rules.push(FirewallRule {
            name: "block-sqlmap".into(),
            pattern: RulePattern::UserAgentMatch("sqlmap".into()),
            action: Action::Deny,
        });

        // Block access to sensitive paths
        for path in &["/.env", "/.git", "/wp-admin", "/phpMyAdmin"] {
            self.rules.push(FirewallRule {
                name: format!("block-sensitive-{}", path),
                pattern: RulePattern::PathPrefix(path.to_string()),
                action: Action::Deny,
            });
        }
    }
}

impl Default for Firewall {
    fn default() -> Self { Self::new() }
}
