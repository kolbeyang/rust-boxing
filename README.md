# ⚙️🥊 Rust Boxing

Little AI boxers trained through reinforcement learning in [Burn](https://burn.dev/), competing live in your browser with Rust and WebAssembly.

**[Live Demo](https://rust-boxing.vercel.app)**

<img width="1424" height="768" alt="image" src="https://github.com/user-attachments/assets/2f4981a8-d589-4b58-ae61-69baf70ac4c2" />

## About

Rust Boxing is a browser-based boxing simulation where AI agents trained with Deep Q-Networks (DQN) compete against each other in real-time. This project was built to explore Rust's capabilities in both machine learning and web development, combining reinforcement learning with high-performance WebAssembly.

The project features 78 trained models, with 8 carefully selected fighters available in the browser demo. Users can spectate matches between different AI opponents, each with unique fighting styles learned through thousands of training episodes.

## Tech Stack

- **Rust** - Core game engine, physics simulation, and training infrastructure
- **Burn** - Deep learning framework for implementing and training DQN models
- **WebAssembly** - Compiles Rust to run at near-native speeds in the browser
- **React + TypeScript** + **Tailwind** - Frontend UI for fighter selection and match visualization
- **Vite** - Fast development and optimized production builds
