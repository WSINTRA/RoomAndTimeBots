
## **🧠 Overview**
The **Archetype Systems framework** is a precision-engineered tool for modeling cognitive archetypes (Jungian, MBTI) as autonomous agents, governed by strict process discipline. It enforces role-based task allocation, time-bound execution, and resource constraints to ensure optimal performance and goal alignment. This system reflects the executive function’s role in **planning, monitoring, and coordinating** complex cognitive processes.

> *"Clarity of purpose, precision of execution—this is the framework’s mandate."*

---

## **🛠 Key Features (Process-Driven Architecture)**
1. **Role Assignment Protocol**
   - **MBTI Typing Engine**: Assigns agents to tasks based on cognitive strengths (e.g., `ISTJ` for logistics, `INTP` for analysis).
   - **Ephemeral Memory**: Agents use transient memory to avoid data bloat, resetting after each simulation.

2. **Time-Bound Execution**
   - **23-Minute Loop**: Simulations terminate after 23 iterations (minutes), triggering a reset if objectives remain unmet.
   - **Timeout Enforcement**: If agents exceed time limits, the system triggers a **panic!** and logs the failure.

3. **Resource Control**
   - **Sandboxed LLM Access**: Ollama integrations are confined to predefined tasks, preventing bias creep.
   - **Rust Efficiency**: Zero-cost abstractions ensure low overhead, high reliability.

---
## **🧩 Requirements**
- **Rust**: Ensure you have the latest version of Rust installed. You can install it from [rustup.rs](https://rustup.rs/).
- **Ollama**: Install the Ollama CLI from [ollama.com](https://ollama.com/).

## **🚀 Usage (Step-by-Step Execution)**
  - Ensure you have an ollama server running, follow ollama instructions if you need help.
  - Install a model and update the model variable in the code.
  - Feel free to play around with the system prompts and the bot personalities.

```bash
cargo run <duration in minutes> "Some Prompt to ruminate on"
```


