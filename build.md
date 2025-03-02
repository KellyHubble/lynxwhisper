# Build Guide for LynxWhisper

## Build Requirements

To compile LynxWhisper, ensure you have:

- **Rust**: Install via `rustup` or something like that.
  
- **Linux**:
    - `libxdo-dex`: Required for X11 keyboard simulation.
        ```bash
        sudo apt update && sudo apt install libxdo-dev  # Ubuntu/Debian
        sudo dnf install libxdo-devel                  # Fedora
        sudo pacman -S libxdo                          # Arch
        ```
    - `libasound2-dev`: Required for ALSA audio input.
        ```bash
        sudo apt install libasound2-dev  # Ubuntu/Debian
        ```
- **macOS**: No extra dependencies typically needed.
- **Windows**: No extra dependencies typically needed.


## Build Steps

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/KellyHubble/lynxwhisper
   cd lynxwhisper
   ```
2. Install Build Dependencies:
   ```bash
   sudo apt update
   sudo apt install libxdo-dev libasound2-dev  # Ubuntu/Debian
   ```
3. **Build LynxWhisper**:
   ```bash
   cargo build --release
   ```
   - The binary will be available at `target/release/lynxwhisper`.


#### Troubleshooting Section (Updated)

## Troubleshooting

- **Linker Error: `cannot find -lxdo`**: Install `libxdo-dev` (see Build Requirements).
- **Linker Error: `cannot find -lasound`**: Install `libasound2-dev`.
- **No Audio**: Check microphone permissions and ensure it’s recognized by your system (e.g., `arecord -l` on Linux).
- **Model Not Found**: Verify the `path` in `config.toml` points to a valid GGML Whisper model file.
- **Hotkeys Not Working**: Ensure no other app is grabbing the key combos (e.g., `Ctrl+Shift+S`).