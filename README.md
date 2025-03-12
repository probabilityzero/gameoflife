# Game of Life 

This app simulates Conway's Game of Life in your terminal. It uses **crossterm** for terminal manipulation (cursor, color, input) and **rand** for initializing a random grid.
It's key features are:

- **Random Grid:** Each cell starts with a 30% chance of being alive.
- **Toroidal Wrapping:** Edges wrap around for neighbor calculations.
- **Generation Updates:** New generations follow classic Game of Life rules.
- **Terminal Rendering:** Live cells display with a white background; dead cells with black.
- **User Input:** Press 'q' to exit.  
- **Alternate Screen:** Uses an alternate screen buffer to keep your terminal clean.

---

### How to Run
1. **Build:** `cargo build --release`
2. **Execute:** Run the binary.
3. **Exit:** Press `q` to quit.

#### Dependencies
- **crossterm**
- **rand**