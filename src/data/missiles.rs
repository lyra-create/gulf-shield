/// Missile type with public-source performance data (IISS, CSIS).
#[derive(Clone, Debug)]
pub struct Missile {
    pub name: &'static str,
    pub range_km: u32,
    pub flight_time_min: u32,
    /// Terminal velocity category affects debris scatter after interception.
    pub terminal_speed_category: TerminalSpeed,
}

#[derive(Clone, Debug)]
pub enum TerminalSpeed {
    /// Short-range, lower terminal velocity — tighter debris field
    Subsonic,
    /// Medium-range — moderate debris scatter
    Supersonic,
    /// Long-range MRBM — high terminal velocity, wider debris scatter
    Hypersonic,
}

impl Missile {
    /// Debris ellipse semi-major axis (along trajectory) in metres after interception.
    /// Based on altitude of intercept, terminal speed, and warhead mass.
    /// Sources: CSIS Missile Defense Project analysis, public DoD reports.
    pub fn intercept_debris_length_m(&self) -> u32 {
        match self.terminal_speed_category {
            TerminalSpeed::Subsonic => 2000,   // ~2km along path
            TerminalSpeed::Supersonic => 4000, // ~4km along path
            TerminalSpeed::Hypersonic => 6000, // ~6km along path
        }
    }

    /// Debris ellipse semi-minor axis (perpendicular to trajectory) in metres.
    /// Wind drift and fragmentation pattern determine lateral spread.
    pub fn intercept_debris_width_m(&self) -> u32 {
        match self.terminal_speed_category {
            TerminalSpeed::Subsonic => 800,
            TerminalSpeed::Supersonic => 1500,
            TerminalSpeed::Hypersonic => 2500,
        }
    }

    /// Typical interception altitude in km (affects fall time and scatter).
    pub fn intercept_altitude_km(&self) -> u32 {
        match self.terminal_speed_category {
            TerminalSpeed::Subsonic => 15,    // Patriot PAC-3 engagement
            TerminalSpeed::Supersonic => 40,  // THAAD lower envelope
            TerminalSpeed::Hypersonic => 100, // THAAD upper envelope
        }
    }
}

pub fn missiles() -> Vec<Missile> {
    vec![
        Missile {
            name: "Fateh-110",
            range_km: 300,
            flight_time_min: 8,
            terminal_speed_category: TerminalSpeed::Subsonic,
        },
        Missile {
            name: "Dezful",
            range_km: 1000,
            flight_time_min: 9,
            terminal_speed_category: TerminalSpeed::Supersonic,
        },
        Missile {
            name: "Shahab-3",
            range_km: 1300,
            flight_time_min: 11,
            terminal_speed_category: TerminalSpeed::Supersonic,
        },
        Missile {
            name: "Emad",
            range_km: 1700,
            flight_time_min: 12,
            terminal_speed_category: TerminalSpeed::Hypersonic,
        },
        Missile {
            name: "Khorramshahr",
            range_km: 2000,
            flight_time_min: 14,
            terminal_speed_category: TerminalSpeed::Hypersonic,
        },
    ]
}
