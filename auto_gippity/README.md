# AutoGPT Gen AI Instrument with Rust

A sophisticated multi-agent AI system built with Rust that orchestrates autonomous agents to design and develop web server applications using GPT-4 integration.

## 🚀 Overview

This project implements an AutoGPT-like architecture where specialized AI agents collaborate to transform user requirements into fully functional web applications. The system leverages Rust's performance, safety, and concurrency features to create a robust, scalable AI orchestration platform.

### Key Capabilities

- **Autonomous Agent Orchestration**: Multi-agent system with specialized roles (Project Manager, Solution Architect, Backend Developer)
- **Dynamic Code Generation**: AI-powered backend code generation with automatic bug fixing and optimization
- **Web Application Scaffolding**: Automated creation of Actix-web based REST APIs with CRUD operations
- **Safety-First Approach**: Built-in code review and safety checks before execution
- **Extensible Architecture**: Modular design supporting custom agents and AI functions

## 🏗️ Architecture

### Agent Hierarchy

```
ManagingAgent (Project Manager)
├── AgentSolutionArchitect
│   ├── Project scope analysis
│   ├── External API requirements detection
│   └── System architecture planning
└── AgentBackendDeveloper
    ├── Code generation from templates
    ├── Bug detection and fixing
    └── API endpoint extraction
```

### Core Components

- **AI Functions**: Declarative AI function definitions with macro-based code generation
- **Agent System**: Trait-based agent architecture with async execution
- **LLM Integration**: Robust OpenAI GPT-4 integration with error handling and retries
- **Code Templates**: Reusable web server templates for rapid prototyping
- **Safety Layer**: Human-in-the-loop validation for AI-generated code execution

## 🛠️ Technology Stack

- **Language**: Rust
- **Async Runtime**: Tokio
- **Web Framework**: Actix-web
- **HTTP Client**: Reqwest
- **Serialization**: Serde
- **AI Integration**: OpenAI GPT-4 API
- **Macros**: Custom procedural macros for AI function definitions

## 📁 Project Structure

```
├── auto_gippity/              # Core AI agent system
│   ├── src/
│   │   ├── apis/              # LLM API integration
│   │   │   └── call_request.rs
│   │   ├── ai_functions/      # AI function definitions
│   │   │   ├── ai_func_managing.rs
│   │   │   ├── ai_func_architect.rs
│   │   │   └── ai_func_backend.rs
│   │   ├── helpers/           # Utilities and CLI
│   │   │   ├── general.rs     # Core helper functions
│   │   │   └── cli.rs         # Command-line interface
│   │   ├── models/            # Data models and agents
│   │   │   ├── agents/        # Agent implementations
│   │   │   ├── agents_manager/ # Agent orchestration
│   │   │   ├── agent_basic/   # Base agent traits
│   │   │   └── general/       # Shared models
│   │   └── lib.rs
│   ├── schemas/               # JSON schemas
│   └── Cargo.toml
├── web_template/              # Generated web application template
│   ├── src/
│   │   ├── code_template.rs   # CRUD and authentication boilerplate
│   │   └── main.rs            # Generated application entry point
│   └── Cargo.toml
├── playaround/               # Rust learning playground
│   └── src/                  # Language feature examples
├── my_proc_macro/            # Custom procedural macros
└── Cargo.toml                # Workspace configuration
```

## 🚀 Getting Started

### Prerequisites

- Rust (1.70+)
- OpenAI API account with GPT-4 access
- Git

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/mutabay/AutoGPT-Gen-AI-Instrument-with-Rust.git
   cd AutoGPT-Gen-AI-Instrument-with-Rust
   ```

2. **Configure environment variables**
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` with your OpenAI credentials:
   ```env
   OPENAI_API_KEY=your_openai_api_key_here
   OPENAI_MODEL=gpt-4
   OPENAI_ORG=your_organization_id
   ```

3. **Build the workspace**
   ```bash
   cargo build --workspace
   ```

4. **Run tests**
   ```bash
   cargo test --workspace
   ```

### Quick Start

Generate a web application from a natural language description:

```bash
cargo run -p auto_gippity
```

Example prompts:
- "Build a fitness tracking app that fetches timezone data from external APIs"
- "Create a TODO application with user authentication and CRUD operations"
- "Develop a cryptocurrency price tracker with real-time data"

## 📚 Usage Examples

### Basic Agent Execution

```rust
use auto_gippity::models::agents_manager::managing_agent::ManagingAgent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_request = "Build a fitness tracking app with timezone support";
    
    let mut managing_agent = ManagingAgent::new(user_request.to_string()).await?;
    managing_agent.execute_project().await;
    
    println!("Project generated successfully!");
    Ok(())
}
```

### Custom AI Functions

```rust
use ai_functions::ai_function;

#[ai_function]
pub fn analyze_requirements(description: &str) {
    /// Analyzes user requirements and extracts key features
    /// Input: Project description string
    /// Output: JSON with required features and complexity analysis
    println!(OUTPUT)
}
```

## 🔧 Configuration

### Agent Behavior

Agents follow a state machine pattern:
- `Discovery`: Initial analysis and planning
- `Working`: Code generation and development
- `UnitTesting`: Validation and safety checks
- `Finished`: Project completion

### Safety Features

- **Human-in-the-loop**: Manual approval required before code execution
- **Code Validation**: Syntax and safety checks on generated code
- **Error Recovery**: Automatic retry mechanisms for failed operations
- **Incremental Development**: Step-by-step validation and refinement

## 🧪 Testing

The project includes comprehensive test coverage:

```bash
# Run all tests
cargo test --workspace

# Run specific agent tests
cargo test -p auto_gippity agent_backend

# Run with output
cargo test -- --nocapture
```


## 📈 Roadmap

- [ ] Frontend code generation (React/Vue.js)
- [ ] Database integration (PostgreSQL, MongoDB)
- [ ] Deployment automation (Docker, Kubernetes)
- [ ] Visual agent workflow designer
- [ ] Multi-language support (Python, TypeScript)
- [ ] Advanced testing agent for generated code



---

**Built with ❤️ using Rust and AI**
