// GUL Programming Language Official Website
// Built with Dioxus 0.5

#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn main() {
    launch(App);
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/docs")]
        Docs {},
        #[route("/blog")]
        Blog {},
        #[route("/community")]
        Community {},
        #[route("/download")]
        Download {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        div { class: "app",
            Header {}
            main { class: "main-content",
                Outlet::<Route> {}
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
                Link { to: Route::Home {}, class: "logo-link",
                    div { class: "logo",
                        h1 { "GUL" }
                        span { class: "tagline", "Universal Language" }
                    }
                }
                ul { class: "nav-links",
                    li { Link { to: Route::Home {}, "Home" } }
                    li { Link { to: Route::Docs {}, "Docs" } }
                    li { Link { to: Route::Blog {}, "Blog" } }
                    li { Link { to: Route::Community {}, "Community" } }
                    li { Link { to: Route::Download {}, class: "download-btn", "Download" } }
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
        Features {}
        QuickStart {}
        CallToAction {}
    }
}

#[component]
fn Docs() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { "Documentation" }
            p { "Coming soon: Comprehensive documentation for GUL." }
            div { class: "docs-grid",
                div { class: "doc-card", h3 { "Getting Started" }, p { "Learn how to install and run GUL." } }
                div { class: "doc-card", h3 { "Language Reference" }, p { "Detailed syntax and feature guide." } }
                div { class: "doc-card", h3 { "Standard Library" }, p { "API reference for the standard library." } }
            }
        }
    }
}

#[component]
fn Blog() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { "Blog" }
            p { "Latest news and updates from the GUL team." }
            div { class: "blog-list",
                div { class: "blog-post",
                    h3 { "GUL v0.12.0 Released" }
                    span { class: "date", "December 1, 2025" }
                    p { "We are excited to announce the release of GUL v0.12.0 with complete IDE support!" }
                }
            }
        }
    }
}

#[component]
fn Community() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { "Community" }
            p { "Join the GUL community." }
            div { class: "community-links",
                a { href: "https://discord.gg/gul", class: "community-link", "Discord Server" }
                a { href: "https://github.com/gul-lang", class: "community-link", "GitHub Repository" }
                a { href: "https://twitter.com/gul_lang", class: "community-link", "Twitter" }
            }
        }
    }
}

#[component]
fn Download() -> Element {
    rsx! {
        div { class: "page-container",
            h1 { "Download GUL" }
            p { "Get the latest version of GUL for your platform." }
            div { class: "download-options",
                div { class: "download-card",
                    h3 { "Linux" }
                    code { "curl -sSf https://gul-lang.org/install.sh | sh" }
                }
                div { class: "download-card",
                    h3 { "macOS" }
                    code { "brew install gul-lang" }
                }
                div { class: "download-card",
                    h3 { "Windows" }
                    code { "winget install gul-lang" }
                }
            }
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "page-container",
            h1 { "Page Not Found" }
            p { "The page you requested does not exist." }
            Link { to: Route::Home {}, "Go back home" }
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
                    Link { to: Route::Docs {}, class: "btn btn-primary", "Get Started" }
                    Link { to: Route::Home {}, class: "btn btn-secondary", "Try Online" }
                }
            }
            div { class: "hero-code",
                pre { class: "code-block",
                    code {
                        "main:\n"
                        "    # Multi-language integration in one file\n"
                        "    \n"
                        "    # Rust for performance\n"
                        "    @rust\n"
                        "    fn fibonacci(n: u64) -> u64 {{\n"
                        "        match n {{\n"
                        "            0 => 0,\n"
                        "            1 => 1,\n"
                        "            _ => fibonacci(n-1) + fibonacci(n-2)\n"
                        "        }}\n"
                        "    }}\n"
                        "    \n"
                        "    # Python for data science\n"
                        "    @python\n"
                        "    import numpy as np\n"
                        "    data = np.array([1, 2, 3, 4, 5])\n"
                        "    \n"
                        "    # JavaScript for web\n"
                        "    @js\n"
                        "    console.log(\"Hello from GUL!\");\n"
                    }
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
                div { class: "feature-card",
                    div { class: "feature-icon", "🚀" }
                    h3 { class: "feature-title", "Multi-Language Integration" }
                    p { class: "feature-description",
                        "Seamlessly mix Rust, Python, JavaScript, C, and SQL in a single codebase with zero-copy data sharing."
                    }
                }
                div { class: "feature-card",
                    div { class: "feature-icon", "⚡" }
                    h3 { class: "feature-title", "High Performance" }
                    p { class: "feature-description",
                        "Compile to native code, WebAssembly, or embedded targets. Optimized for speed and efficiency."
                    }
                }
                div { class: "feature-card",
                    div { class: "feature-icon", "🎯" }
                    h3 { class: "feature-title", "Type Safety" }
                    p { class: "feature-description",
                        "Advanced type system with inference, generics, and compile-time checks for bulletproof code."
                    }
                }
                div { class: "feature-card",
                    div { class: "feature-icon", "🌐" }
                    h3 { class: "feature-title", "Cross-Platform" }
                    p { class: "feature-description",
                        "Build for desktop, web, mobile, and embedded systems from a single codebase."
                    }
                }
                div { class: "feature-card",
                    div { class: "feature-icon", "🤖" }
                    h3 { class: "feature-title", "AI-Powered" }
                    p { class: "feature-description",
                        "Built-in AI assistant for code generation, refactoring, and intelligent suggestions."
                    }
                }
                div { class: "feature-card",
                    div { class: "feature-icon", "🔧" }
                    h3 { class: "feature-title", "Modern Tooling" }
                    p { class: "feature-description",
                        "Integrated formatter, linter, debugger, and package manager for a complete development experience."
                    }
                }
            }
        }
    }
}

#[component]
fn QuickStart() -> Element {
    rsx! {
        section { class: "quick-start",
            h2 { class: "section-title", "Quick Start" }
            div { class: "quick-start-steps",
                div { class: "step",
                    div { class: "step-number", "1" }
                    h3 { class: "step-title", "Install GUL" }
                    pre { class: "code-block",
                        code { "curl -sSf https://gul-lang.org/install.sh | sh" }
                    }
                }
                div { class: "step",
                    div { class: "step-number", "2" }
                    h3 { class: "step-title", "Create a Project" }
                    pre { class: "code-block",
                        code { "gul new my-project\ncd my-project" }
                    }
                }
                div { class: "step",
                    div { class: "step-number", "3" }
                    h3 { class: "step-title", "Run Your Code" }
                    pre { class: "code-block",
                        code { "gul run main.gul" }
                    }
                }
            }
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
                Link { to: Route::Docs {}, class: "btn btn-large btn-secondary", "Read the Docs" }
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
                    p { class: "version", "Version 0.12.0" }
                }
                div { class: "footer-section",
                    h4 { "Resources" }
                    ul {
                        li { Link { to: Route::Docs {}, "Documentation" } }
                        li { Link { to: Route::Docs {}, "Tutorials" } }
                        li { Link { to: Route::Home {}, "Playground" } }
                        li { a { href: "https://github.com/gul-lang", target: "_blank", "GitHub" } }
                    }
                }
                div { class: "footer-section",
                    h4 { "Community" }
                    ul {
                        li { a { href: "https://discord.gg/gul", target: "_blank", "Discord" } }
                        li { a { href: "https://reddit.com/r/gul", target: "_blank", "Reddit" } }
                        li { a { href: "https://twitter.com/gul_lang", target: "_blank", "Twitter" } }
                        li { Link { to: Route::Community {}, "Forum" } }
                    }
                }
                div { class: "footer-section",
                    h4 { "Legal" }
                    ul {
                        li { a { href: "#privacy", "Privacy Policy" } }
                        li { a { href: "#terms", "Terms of Service" } }
                        li { a { href: "#license", "License" } }
                    }
                }
            }
            div { class: "footer-bottom",
                p { "© 2025 GUL Programming Language. All rights reserved." }
            }
        }
    }
}
