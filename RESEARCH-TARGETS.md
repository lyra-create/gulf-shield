# Gulf Shield — Target Analysis & Debris Corridor Research

_Compiled Feb 28, 2026 from breaking news + public military sources_

## Confirmed/Reported Iranian Targets in UAE (Feb 28, 2026)

### Primary Target: Al Dhafra Air Base
- **Location:** ~24.248°N, 54.548°E (near Abu Dhabi)
- **Why:** Hosts US Air Force 380th Expeditionary Wing (F-35A, F-22, KC-10, RQ-4, U-2)
- **Evidence:** IRGC explicitly named it as part of "True Promise 4" operation
- **Status:** At least 2 ballistic missiles intercepted directly over the base (THAAD engagement)
- **Debris fell:** Saadiyat Island, Khalifa City, Bani Yas, MBZ City, Al Falah area

### Secondary Target: Al Minhad Air Base
- **Location:** ~25.028°N, 55.363°E (Dubai, near Al Ain Road)
- **Why:** US Army 1st Theater Sustainment Command logistics hub, C-130 operations
- **Evidence:** Known US facility, consistent with Iran targeting all US-linked sites in Gulf
- **Status:** No confirmed strike reports yet, but within attack profile

### Naval Target: Jebel Ali Port
- **Location:** ~25.007°N, 55.060°E (Dubai)
- **Why:** US Navy deep-water port, Nimitz-class carrier maintenance, 5th Fleet support
- **Evidence:** Residents reported distant booms in Jebel Ali area; port hosts US naval vessels regularly
- **Status:** No confirmed strikes, but reports of explosions heard in area

### Collateral/Debris Impact: Palm Jumeirah
- **Location:** ~25.112°N, 55.138°E
- **What happened:** Fire at Fairmont Hotel area, 4 injuries
- **Likely cause:** Intercepted missile debris or drone (Shahed-136) that wasn't the primary target
- **Note:** Palm Jumeirah is NOT a military target — this was debris/spillover

### Airports (Precautionary Closure)
- **DXB** (Dubai International) — closed
- **DWC** (Al Maktoum International) — closed
- **AUH** (Abu Dhabi International) — closed

## Approach Vectors (Iran → UAE)

Missiles launched from western/central Iran approach the UAE from the **north-northeast**:
- Tehran → Abu Dhabi: ~1,800km, bearing ~195° (roughly SSW)
- Tabriz/Isfahan → Abu Dhabi: similar bearing
- Approach crosses the Strait of Hormuz, then descends over northern UAE

This means:
- **Debris from interceptions over Abu Dhabi** falls in a corridor NNE→SSW along the incoming path
- **Debris from interceptions over Dubai** similarly follows a NNE→SSW scatter pattern
- The interception altitude determines how far the debris spreads from the interception point

## Debris Corridor Logic for the App

**Current model problem:** We modeled debris as ellipses at a fixed point (85% of trajectory). 

**Better model — what George is asking for:**
1. Draw the **full incoming trajectory** from launch site to target
2. The interception can happen anywhere in the terminal phase (last ~20% of flight path)
3. Model the **debris corridor** as an elongated zone along the final approach bearing
4. Corridor width depends on engagement altitude (THAAD = wider scatter from higher alt, Patriot = narrower)
5. Use proper GIS (Turf.js `buffer`, `bearing`, `destination`, `ellipse`) for geodesic accuracy

**Real-world confirmation from today:**
- Al Dhafra interceptions → debris fell across 5 residential areas of Abu Dhabi, all roughly south/southwest of the base
- Dubai interceptions → debris fell on Palm Jumeirah (which sits southwest of Al Minhad's bearing from Iran)
- Pattern: debris scatters BEYOND the target, along the continuation of the incoming trajectory

## US Military Presence Summary (for targets data)

| Base | Type | Location | US Units | Coords |
|------|------|----------|----------|--------|
| Al Dhafra AB | Air Force | Nr Abu Dhabi | 380th AEW (F-35A, F-22, U-2) | 24.248, 54.548 |
| Al Minhad AB | Army/Logistics | Dubai | 1st TSC (C-130) | 25.028, 55.363 |
| Jebel Ali Port | Navy | Dubai | 5th Fleet support | 25.007, 55.060 |
| Fujairah Naval | Navy | Fujairah | Naval logistics | 25.152, 56.356 |

## Missile Types Used (reported)
- **Ballistic missiles** (likely Emad, Sejjil, or Fattah-1 class — range 1,500-2,000km)
- **Shahed-136 drones** (reported at Palm Jumeirah — cruise, not ballistic)
- Multiple waves — at least 2 confirmed by UAE MoD

## Sources
- WIRED (Feb 28, 2026) — THAAD/defense system analysis
- Khaleej Times (Feb 28, 2026) — debris locations in Abu Dhabi
- Hindustan Times (Feb 28, 2026) — full list of explosion locations in Dubai
- United24 Media (Feb 28, 2026) — Al Dhafra as primary target
- Reuters (Feb 28, 2026) — 1 civilian killed in Abu Dhabi debris
- Washington Times (Feb 28, 2026) — full list of Gulf bases targeted
- MilitaryBaseGuides — US facility details and coordinates
