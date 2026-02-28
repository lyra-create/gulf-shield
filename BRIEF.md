# Gulf Shield — Project Brief

## Overview
A Leptos (Rust/WASM) web app showing potential missile trajectories from Iran to UAE targets, with shrapnel/debris zones, safety procedures, and embassy contacts. **Informational and calming** — NOT alarmist. Think BBC emergency info page, not war propaganda.

## Tech Stack
- **Leptos 0.7+** (CSR mode, compiled to WASM via Trunk)
- **Leaflet.js** via JS interop (`wasm-bindgen`, `js-sys`, `web-sys`) for interactive map
- **Trunk** for build/serve
- Target: `wasm32-unknown-unknown`

## Architecture

### Single-page app with these sections:

1. **Interactive Map (main view, ~70% of screen)**
   - Leaflet map centered on the Gulf region (lat 24.5, lng 54.5, zoom 7)
   - Use OpenStreetMap tiles (free, no API key)
   - Iranian launch sites (publicly documented): Isfahan, Tabriz, Shiraz, Bandar Abbas, Kermanshah
   - UAE target areas: Abu Dhabi, Dubai, Al Dhafra, Fujairah
   - **Trajectory arcs**: Great circle curves from launch sites to targets — use dashed lines, muted blue/grey color
   - **Shrapnel/debris zones**: Semi-transparent ellipses around each target. Use publicly known CEP (Circular Error Probable) data:
     - Short-range ballistic (Fateh-110): ~300m CEP → show 1km debris radius
     - Medium-range (Shahab-3/Emad): ~500m CEP → show 2km debris radius  
     - Long-range (Khorramshahr): ~800m CEP → show 3km debris radius
   - Color code debris zones: light amber at center, fading to transparent at edges
   - Clicking a trajectory shows missile type, estimated flight time, and range

2. **Safety Panel (right sidebar or bottom drawer on mobile)**
   - **What to do during an alert**: Get indoors, stay away from windows, go to lowest floor, interior room
   - **UAE emergency numbers**: 999 (police), 998 (ambulance), 997 (civil defense)
   - **NCEMA guidance**: Link to ncema.gov.ae
   - **Shelter locations**: Note that UAE has designated shelters in malls and metro stations

3. **Embassy Directory (expandable section)**
   - Major embassies in UAE with phone numbers and emergency hotlines
   - UK, US, India, Pakistan, Philippines, Australia, Canada, France, Germany
   - Each with a "Visit website" link
   - Note: "Contact your embassy for country-specific evacuation advice"

4. **Information Footer**
   - "Data is approximate and for informational purposes only"
   - "Follow official NCEMA guidance"
   - "This tool does not predict attacks — it shows potential scenarios to help you prepare"
   - Link to NCEMA app download

## Design Guidelines

### Color Palette (calming, not alarming)
- Background: `#f8f9fa` (light grey)
- Primary text: `#2c3e50` (dark blue-grey)
- Accent: `#3498db` (calm blue)
- Map trajectories: `#7f8c8d` (grey) with `#2980b9` (blue) on hover
- Debris zones: `rgba(230, 180, 60, 0.15)` center → `rgba(230, 180, 60, 0.05)` edge
- Safety panel: `#ecf0f1` background
- NO RED anywhere in the UI. Use amber/gold sparingly for debris zones only.

### Typography
- System font stack: `-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif`
- Clean, readable, generous line-height (1.6)
- Header: "Gulf Shield" with a small shield icon (Unicode ️🛡️)

### Responsive
- Desktop: Map left (70%), safety panel right (30%)
- Mobile: Map full width, safety panel as bottom drawer

## File Structure
```
gulf-shield/
├── Cargo.toml          # Leptos + wasm-bindgen deps
├── Trunk.toml          # Trunk config
├── index.html          # Entry point with Leaflet CSS/JS CDN links
├── src/
│   ├── main.rs         # App entry, mount Leptos
│   ├── app.rs          # Main App component
│   ├── components/
│   │   ├── mod.rs
│   │   ├── map.rs      # Leaflet map component (JS interop)
│   │   ├── safety.rs   # Safety procedures panel
│   │   ├── embassy.rs  # Embassy directory
│   │   └── info.rs     # Missile info popup content
│   ├── data/
│   │   ├── mod.rs
│   │   ├── sites.rs    # Launch sites & targets (lat/lng)
│   │   ├── missiles.rs # Missile types, ranges, CEP
│   │   └── embassies.rs # Embassy contact info
│   └── js_bindings.rs  # wasm-bindgen bindings for Leaflet
├── style/
│   └── main.css        # All styles
└── BRIEF.md            # This file
```

## Leaflet JS Interop Pattern

Use `wasm-bindgen` + `js_sys` + `web_sys` to call Leaflet. Key pattern:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = L)]
    fn map(id: &str) -> JsValue;
    
    // etc.
}
```

Or use inline JS with `#[wasm_bindgen(inline_js = "...")]` for the map initialization — this is often simpler for Leaflet.

**Recommended approach**: Create a `<div id="map">` in the Leptos component, then use `create_effect` to run JS that initializes Leaflet after mount. Use `js_sys::eval()` or a dedicated JS snippet file for the map logic.

## Launch Site Data (public sources: CSIS Missile Threat Project)

```rust
// Iranian launch sites (approximate, publicly documented)
("Isfahan", 32.6546, 51.6680),      // Central Iran
("Tabriz", 38.0800, 46.2919),       // Northwest
("Shiraz", 29.5918, 52.5837),       // South
("Bandar Abbas", 27.1865, 56.2808), // Strait of Hormuz
("Kermanshah", 34.3142, 47.0650),   // Western Iran
("Semnan", 35.5769, 53.3970),       // Test range

// UAE targets (population/infrastructure centers)
("Abu Dhabi", 24.4539, 54.3773),
("Dubai", 25.2048, 55.2708),
("Al Dhafra Air Base", 24.2483, 54.5475),
("Fujairah", 25.1288, 56.3264),
("Jebel Ali", 24.9857, 55.0272),
```

## Missile Data (public sources: IISS, CSIS)

```rust
("Fateh-110", 300, 300, 8),     // range_km, cep_m, flight_time_min
("Shahab-3", 1300, 500, 11),
("Emad", 1700, 500, 12),
("Khorramshahr", 2000, 800, 14),
("Dezful", 1000, 300, 9),
```

## Build & Serve
```bash
source "$HOME/.cargo/env"
cd /home/ubuntu/.openclaw/workspace/repos/gulf-shield
trunk serve --address 0.0.0.0 --port 8085
```

## After Building
- Commit: `git add -A && git commit -m "feat: initial Gulf Shield app"`
- Verify it compiles: `trunk build`
- Verify it serves: `trunk serve --port 8085`

## IMPORTANT NOTES
- This uses ONLY publicly available, unclassified data from think tanks (CSIS, IISS)
- All distances/trajectories are approximate and clearly labeled as such
- The tone is "preparedness" not "prediction" — we're not saying an attack WILL happen
- No countdown timers, no sirens, no flashing — just calm, clear information
