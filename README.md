# 🎙️ Logium

A custom, lightweight, system-tray Windows application built to replace Logitech G HUB for the **Logitech Yeti Orb** microphone. 

Logium bypasses heavy background bloat by bridging a high-performance Rust DSP audio pipeline with a sleek, frameless React frontend that docks directly above your Windows system tray.

### 🚀 Features
* **Smart Auto-Tuner:** A background acoustic classifier analyzes RMS energy and Zero-Crossing Rates in real-time, automatically hot-swapping between `QuietStudio`, `VoiceFocused`, and `NoisyEnvironment` profiles based on your room noise.
* **Custom DSP Engine:** Built on WASAPI and `cpal`, featuring a zero-latency audio pipeline with High-Pass Filters, 3-Band Parametric EQ, downward noise expansion, and hard limiters.
* **Tactile System Tray UI:** A frameless Tauri v2 widget built with React, Tailwind CSS v4, and Framer Motion spring physics.
* **Hardware Sidetone:** Instantly toggle zero-latency audio monitoring to test your EQ configurations.

### 🛠️ Tech Stack
* **Core / Audio Engine:** Rust (`cpal`, `biquad`, `hidapi`)
* **Framework:** Tauri v2
* **Frontend:** React, TypeScript, Vite
* **Styling & Animation:** Tailwind CSS v4, Framer Motion, Lucide React

### 💻 Local Development
To run this project locally, you need [Node.js](https://nodejs.org/), [pnpm](https://pnpm.io/), and the [Rust Toolchain](https://rustup.rs/) installed.

1. **Clone the repository:**
   `git clone https://github.com/debaditya99/logium.git`
2. **Install frontend dependencies:**
   `pnpm install`
3. **Run the development server:**
   `pnpm run tauri dev`

### 🤝 How to Contribute
We welcome contributions to make Logium even better! All active development happens on the `test` branch. 

To contribute:
1. Fork this repository and clone it locally.
2. Create a new branch from the `test` branch (do not branch from `main`).
3. Make your changes and test them locally.
4. Push your branch to your fork and open a Pull Request targeting the `test` branch of this original repository[cite: 5].