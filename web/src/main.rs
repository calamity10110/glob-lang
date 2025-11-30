// GUL Programming Language Official Website
// Built with Dioxus

#![allow(non_snake_case)]

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/learn")]
    Learn {},
    #[route("/docs")]
    Docs {},
    #[route("/playground")]
    Playground {},
    #[route("/community")]
    Community {},
    #[route("/download")]
    Download {},
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "app",
            Header {}
            main { class: "main-content",
                Hero {}
                Features {}
                QuickStart {}
                CodeExample {}
                CallToAction {}
            }
            Footer {}
        }
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        header { class: "header",
            nav { class: "nav",
                div { class: "logo",
                    h1 { "GUL" }
                    span { class: "tagline", "Universal Language" }
                }
                ul { class: "nav-links",
                    li { Link { to: Route::Home {}, "Home" } }
                    li { Link { to: Route::Learn {}, "Learn" } }
                    li { Link { to: Route::Docs {}, "Docs" } }
                    li { Link { to: Route::Playground {}, "Playground" } }
                    li { Link { to: Route::Community {}, "Community" } }
                    li { Link { to: Route::Download {}, class: "download-btn", "Download" } }
                }
            }
        }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section { class: "hero",
            div { class: "hero-content",
                h1 { class: "hero-title",
                    "The Universal Programming Language"
                }
                p { class: "hero-subtitle",
                    "Write once, run everywhere. GUL seamlessly integrates Rust, Python, JavaScript, C, and SQL in a single, elegant syntax."
                }
                div { class: "hero-buttons",
                    Link { to: Route::Learn {}, class: "btn btn-primary", "Get Started" }
                    Link { to: Route::Playground {}, class: "btn btn-secondary", "Try Online" }
                }
            }
            div { class: "hero-code",
                CodeBlock {
                    code: r#"main:
    # Multi-language integration in one file
    
    # Rust for performance
    @rust
    fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n-1) + fibonacci(n-2)
        }
    }
    
    # Python for data science
    @python
    import numpy as np
    data = np.array([1, 2, 3, 4, 5])
    
    # JavaScript for web
    @js
    console.log("Hello from GUL!");
    
    # SQL for databases
    @sql
    SELECT * FROM users WHERE active = true;
"#
                }
            }
        }
    }
}

#[component]
fn Features() -> Element {
    rsx! {
        section { class: "features",
            h2 { class: "section-title", "Why Choose GUL?" }
            div { class: "features-grid",
                FeatureCard {
                    icon: "🚀",
                    title: "Multi-Language Integration",
                    description: "Seamlessly mix Rust, Python, JavaScript, C, and SQL in a single codebase with zero-copy data sharing."
                }
                FeatureCard {
                    icon: "⚡",
                    title: "High Performance",
                    description: "Compile to native code, WebAssembly, or embedded targets. Optimized for speed and efficiency."
                }
                FeatureCard {
                    icon: "🎯",
                    title: "Type Safety",
                    description: "Advanced type system with inference, generics, and compile-time checks for bulletproof code."
                }
                FeatureCard {
                    icon: "🌐",
                    title: "Cross-Platform",
                    description: "Build for desktop, web, mobile, and embedded systems from a single codebase."
                }
                FeatureCard {
                    icon: "🤖",
                    title: "AI-Powered",
                    description: "Built-in AI assistant for code generation, refactoring, and intelligent suggestions."
                }
                FeatureCard {
                    icon: "🔧",
                    title: "Modern Tooling",
                    description: "Integrated formatter, linter, debugger, and package manager for a complete development experience."
                }
            }
        }
    }
}

#[component]
fn FeatureCard(icon: &'static str, title: &'static str, description: &'static str) -> Element {
    rsx! {
        div { class: "feature-card",
            div { class: "feature-icon", "{icon}" }
            h3 { class: "feature-title", "{title}" }
            p { class: "feature-description", "{description}" }
        }
    }
}

#[component]
fn QuickStart() -> Element {
    rsx! {
        section { class: "quick-start",
            h2 { class: "section-title", "Quick Start" }
            div { class: "quick-start-steps",
                Step {
                    number: "1",
                    title: "Install GUL",
                    code: "curl -sSf https://gul-lang.org/install.sh | sh"
                }
                Step {
                    number: "2",
                    title: "Create a Project",
                    code: "gul new my-project\ncd my-project"
                }
                Step {
                    number: "3",
                    title: "Run Your Code",
                    code: "gul run main.gul"
                }
            }
        }
    }
}

#[component]
fn Step(number: &'static str, title: &'static str, code: &'static str) -> Element {
    rsx! {
        div { class: "step",
            div { class: "step-number", "{number}" }
            h3 { class: "step-title", "{title}" }
            CodeBlock { code: code }
        }
    }
}

#[component]
fn CodeExample() -> Element {
    rsx! {
        section { class: "code-example",
            h2 { class: "section-title", "See It In Action" }
            div { class: "example-tabs",
                div { class: "tab-content",
                    CodeBlock {
                        code: r#"# Web Server with Database
main:
    # Define data model
    @rust
    #[derive(Serialize, Deserialize)]
    struct User {
        id: i32,
        name: String,
        email: String,
    }
    
    # Database queries
    @sql
    CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        email TEXT UNIQUE
    );
    
    # API endpoints
    @rust
    async fn get_users() -> Vec<User> {
        query!("SELECT * FROM users")
            .fetch_all()
            .await
    }
    
    # Start server
    println("Server running on http://localhost:8080")
    serve(port: 8080)
"#
                    }
                }
            }
        }
    }
}

#[component]
fn CodeBlock(code: &'static str) -> Element {
    rsx! {
        pre { class: "code-block",
            code { "{code}" }
        }
    }
}

#[component]
fn CallToAction() -> Element {
    rsx! {
        section { class: "cta",
            h2 { "Ready to Get Started?" }
            p { "Join thousands of developers building the future with GUL" }
            div { class: "cta-buttons",
                Link { to: Route::Download {}, class: "btn btn-large btn-primary", "Download GUL" }
                Link { to: Route::Learn {}, class: "btn btn-large btn-secondary", "Read the Docs" }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            div { class: "footer-content",
                div { class: "footer-section",
                    h4 { "GUL" }
                    p { "The Universal Programming Language" }
                    p { class: "version", "Version 0.11.0" }
                }
                div { class: "footer-section",
                    h4 { "Resources" }
                    ul {
                        li { Link { to: Route::Docs {}, "Documentation" } }
                        li { Link { to: Route::Learn {}, "Tutorials" } }
                        li { Link { to: Route::Playground {}, "Playground" } }
                        li { a { href: "https://github.com/gul-lang", "GitHub" } }
                    }
                }
                div { class: "footer-section",
                    h4 { "Community" }
                    ul {
                        li { a { href: "https://discord.gg/gul", "Discord" } }
                        li { a { href: "https://reddit.com/r/gul", "Reddit" } }
                        li { a { href: "https://twitter.com/gul_lang", "Twitter" } }
                        li { Link { to: Route::Community {}, "Forum" } }
                    }
                }
                div { class: "footer-section",
                    h4 { "Legal" }
                    ul {
                        li { a { href: "/privacy", "Privacy Policy" } }
                        li { a { href: "/terms", "Terms of Service" } }
                        li { a { href: "/license", "License" } }
                    }
                }
            }
            div { class: "footer-bottom",
                p { "© 2025 GUL Programming Language. All rights reserved." }
            }
        }
    }
}

// Additional pages

#[component]
fn Learn() -> Element {
    rsx! {
        div { class: "app",
            Header {}
            main { class: "main-content learn-page",
                h1 { "Learn GUL" }
                div { class: "learning-paths",
                    LearningPath {
                        title: "Getting Started",
                        description: "New to GUL? Start here to learn the basics.",
                        lessons: vec![
                            "Installation",
                            "Your First Program",
                            "Basic Syntax",
                            "Variables and Types",
                            "Control Flow"
                        ]
                    }
                    LearningPath {
                        title: "Multi-Language Integration",
                        description: "Learn how to integrate multiple languages seamlessly.",
                        lessons: vec![
                            "Rust Integration",
                            "Python Integration",
                            "JavaScript Integration",
                            "C FFI",
                            "SQL Queries"
                        ]
                    }
                    LearningPath {
                        title: "Advanced Topics",
                        description: "Deep dive into advanced GUL features.",
                        lessons: vec![
                            "Async Programming",
                            "Package Management",
                            "Testing",
                            "Deployment",
                            "Performance Optimization"
                        ]
                    }
                }
            }
            Footer {}
        }
    }
}

#[component]
fn LearningPath(
    title: &'static str,
    description: &'static str,
    lessons: Vec<&'static str>,
) -> Element {
    rsx! {
        div { class: "learning-path",
            h2 { "{title}" }
            p { "{description}" }
            ul { class: "lessons",
                for lesson in lessons {
                    li { a { href: "#", "{lesson}" } }
                }
            }
        }
    }
}

#[component]
fn Docs() -> Element {
    rsx! {
        div { class: "app",
            Header {}
            main { class: "main-content docs-page",
                aside { class: "docs-sidebar",
                    nav {
                        h3 { "Documentation" }
                        ul {
                            li { a { href: "#intro", "Introduction" } }
                            li { a { href: "#syntax", "Syntax Guide" } }
                            li { a { href: "#stdlib", "Standard Library" } }
                            li { a { href: "#api", "API Reference" } }
                            li { a { href: "#examples", "Examples" } }
                        }
                    }
                }
                article { class: "docs-content",
                    h1 { "GUL Documentation" }
                    section { id: "intro",
                        h2 { "Introduction" }
                        p { "GUL (Universal Language) is a modern programming language designed for seamless multi-language integration." }
                    }
                    section { id: "syntax",
                        h2 { "Syntax Guide" }
                        p { "GUL uses a clean, Python-inspired syntax with powerful type inference." }
                    }
                }
            }
            Footer {}
        }
    }
}

#[component]
fn Playground() -> Element {
    rsx! {
        div { class: "app",
            Header {}
            main { class: "main-content playground-page",
                h1 { "GUL Playground" }
                div { class: "playground",
                    div { class: "editor-panel",
                        h3 { "Code Editor" }
                        textarea {
                            class: "code-editor",
                            placeholder: "Write your GUL code here...",
                            rows: "20"
                        }
                        button { class: "btn btn-primary", "Run Code" }
                    }
                    div { class: "output-panel",
                        h3 { "Output" }
                        pre { class: "output",
                            "// Output will appear here"
                        }
                    }
                }
            }
            Footer {}
        }
    }
}

#[component]
fn Community() -> Element {
    rsx! {
        div { class: "app",
            Header {}
            main { class: "main-content community-page",
                h1 { "Join the GUL Community" }
                div { class: "community-grid",
                    CommunityCard {
                        icon: "💬",
                        title: "Discord",
                        description: "Chat with other GUL developers in real-time",
                        link: "https://discord.gg/gul"
                    }
                    CommunityCard {
                        icon: "📖",
                        title: "Forum",
                        description: "Ask questions and share knowledge",
                        link: "/forum"
                    }
                    CommunityCard {
                        icon: "🐙",
                        title: "GitHub",
                        description: "Contribute to GUL development",
                        link: "https://github.com/gul-lang"
                    }
                    CommunityCard {
                        icon: "🐦",
                        title: "Twitter",
                        description: "Follow for updates and announcements",
                        link: "https://twitter.com/gul_lang"
                    }
                }
            }
            Footer {}
        }
    }
}

#[component]
fn CommunityCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    link: &'static str,
) -> Element {
    rsx! {
        div { class: "community-card",
            div { class: "community-icon", "{icon}" }
            h3 { "{title}" }
            p { "{description}" }
            a { href: "{link}", class: "btn btn-secondary", "Join" }
        }
    }
}

#[component]
fn Download() -> Element {
    rsx! {
        div { class: "app",
            Header {}
            main { class: "main-content download-page",
                h1 { "Download GUL" }
                div { class: "download-options",
                    DownloadOption {
                        platform: "Linux",
                        icon: "🐧",
                        command: "curl -sSf https://gul-lang.org/install.sh | sh"
                    }
                    DownloadOption {
                        platform: "macOS",
                        icon: "🍎",
                        command: "brew install gul"
                    }
                    DownloadOption {
                        platform: "Windows",
                        icon: "🪟",
                        command: "winget install gul"
                    }
                }
                div { class: "version-info",
                    h2 { "Current Version: 0.11.0" }
                    p { "Released: November 2025" }
                    a { href: "/changelog", "View Changelog" }
                }
            }
            Footer {}
        }
    }
}

#[component]
fn DownloadOption(platform: &'static str, icon: &'static str, command: &'static str) -> Element {
    rsx! {
        div { class: "download-option",
            div { class: "platform-icon", "{icon}" }
            h3 { "{platform}" }
            CodeBlock { code: command }
            button { class: "btn btn-primary", "Copy Command" }
        }
    }
}
