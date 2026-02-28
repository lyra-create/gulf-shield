/// UAE missile defense system locations (publicly documented).
/// Sources: DoD press releases, CSIS Missile Defense Project, news reports.
#[derive(Clone, Debug)]
pub struct DefenseSystem {
    pub name: &'static str,
    pub system_type: DefenseType,
    pub lat: f64,
    pub lng: f64,
    /// Engagement range in km
    pub range_km: u32,
}

#[derive(Clone, Debug)]
pub enum DefenseType {
    /// Terminal High Altitude Area Defense
    Thaad,
    /// Patriot Advanced Capability-3
    PatriotPac3,
}

impl DefenseSystem {
    pub fn type_label(&self) -> &'static str {
        match self.system_type {
            DefenseType::Thaad => "THAAD",
            DefenseType::PatriotPac3 => "Patriot PAC-3",
        }
    }
}

/// Known UAE defense installations (publicly reported).
/// The UAE purchased THAAD in 2011 (delivered ~2015) and operates Patriot batteries.
/// Locations are approximate based on public reporting.
pub fn defense_systems() -> Vec<DefenseSystem> {
    vec![
        DefenseSystem {
            name: "Al Dhafra THAAD",
            system_type: DefenseType::Thaad,
            lat: 24.2483,
            lng: 54.5475,
            range_km: 200,
        },
        DefenseSystem {
            name: "Abu Dhabi Patriot",
            system_type: DefenseType::PatriotPac3,
            lat: 24.42,
            lng: 54.43,
            range_km: 70,
        },
        DefenseSystem {
            name: "Dubai Patriot",
            system_type: DefenseType::PatriotPac3,
            lat: 25.07,
            lng: 55.17,
            range_km: 70,
        },
        DefenseSystem {
            name: "Jebel Ali Patriot",
            system_type: DefenseType::PatriotPac3,
            lat: 24.98,
            lng: 55.03,
            range_km: 70,
        },
    ]
}
