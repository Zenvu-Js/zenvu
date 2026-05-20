//! Abstract Syntax Tree definitions for Zenvu.js components.

use crate::error::Span;

/// Root AST node for a `.Zenvu` component.
#[derive(Debug, Clone)]
pub struct ComponentAst {
    pub name: String,
    pub script: Option<ScriptBlock>,
    pub template: Option<TemplateBlock>,
    pub style: Option<StyleBlock>,
}

#[derive(Debug, Clone)]
pub struct ScriptBlock {
    pub lang: String,
    pub content: String,
    pub declarations: Vec<Declaration>,
    pub imports: Vec<ImportDecl>,
    pub props: Vec<PropDecl>,
    pub emits: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    ReactiveVar { name: String, type_ann: Option<String>, init: Option<String>, span: Option<Span> },
    DerivedVar { name: String, type_ann: Option<String>, expression: String, dependencies: Vec<String>, span: Option<Span> },
    Function { name: String, params: Vec<String>, body: String, is_async: bool, span: Option<Span> },
    LifecycleHook { hook: LifecycleKind, body: String, span: Option<Span> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleKind { OnMount, OnUpdate, OnDestroy, OnError }

#[derive(Debug, Clone)]
pub struct ImportDecl {
    pub specifiers: Vec<ImportSpecifier>,
    pub source: String,
}

#[derive(Debug, Clone)]
pub enum ImportSpecifier {
    Default(String),
    Named { local: String, imported: String },
    Namespace(String),
}

#[derive(Debug, Clone)]
pub struct PropDecl {
    pub name: String,
    pub type_ann: Option<String>,
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TemplateBlock {
    pub children: Vec<TemplateNode>,
}

#[derive(Debug, Clone)]
pub enum TemplateNode {
    Element(ElementNode),
    Text(TextNode),
    Interpolation(InterpolationNode),
    Comment(String),
    ConditionalBlock(ConditionalNode),
    ForBlock(ForNode),
}

#[derive(Debug, Clone)]
pub struct ElementNode {
    pub tag: String,
    pub is_component: bool,
    pub attributes: Vec<Attribute>,
    pub directives: Vec<Directive>,
    pub children: Vec<TemplateNode>,
    pub self_closing: bool,
    pub key: Option<String>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone)]
pub struct Attribute { pub name: String, pub value: Option<String> }

#[derive(Debug, Clone)]
pub struct Directive {
    pub kind: DirectiveKind,
    pub argument: Option<String>,
    pub modifiers: Vec<String>,
    pub expression: Option<String>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectiveKind {
    Bind, On, If, ElseIf, Else, For, Model, Show, Transition, Html, TextDirective, Ref, Slot,
}

#[derive(Debug, Clone)]
pub struct TextNode { pub content: String, pub span: Option<Span> }

#[derive(Debug, Clone)]
pub struct InterpolationNode {
    pub expression: String,
    pub dependencies: Vec<String>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone)]
pub struct ConditionalNode { pub branches: Vec<ConditionalBranch> }

#[derive(Debug, Clone)]
pub struct ConditionalBranch {
    pub condition: Option<String>,
    pub children: Vec<TemplateNode>,
}

#[derive(Debug, Clone)]
pub struct ForNode {
    pub value_alias: String,
    pub index_alias: Option<String>,
    pub source: String,
    pub key: Option<String>,
    pub children: Vec<TemplateNode>,
}

#[derive(Debug, Clone)]
pub struct StyleBlock {
    pub scoped: bool,
    pub lang: Option<String>,
    pub content: String,
}

impl ElementNode {
    pub fn has_directive(&self, kind: &DirectiveKind) -> bool {
        self.directives.iter().any(|d| &d.kind == kind)
    }
    pub fn get_directive(&self, kind: &DirectiveKind) -> Option<&Directive> {
        self.directives.iter().find(|d| &d.kind == kind)
    }
}

impl ComponentAst {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), script: None, template: None, style: None }
    }
}
