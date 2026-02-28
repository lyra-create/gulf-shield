/// A geographic site (launch origin or target).
#[derive(Clone, Debug)]
pub struct Site {
    pub name: &'static str,
    pub lat: f64,
    pub lng: f64,
}

/// Iranian launch sites (approximate, publicly documented — CSIS Missile Threat Project).
pub fn launch_sites() -> Vec<Site> {
    vec![
        Site {
            name: "Isfahan",
            lat: 32.6546,
            lng: 51.6680,
        },
        Site {
            name: "Tabriz",
            lat: 38.0800,
            lng: 46.2919,
        },
        Site {
            name: "Shiraz",
            lat: 29.5918,
            lng: 52.5837,
        },
        Site {
            name: "Bandar Abbas",
            lat: 27.1865,
            lng: 56.2808,
        },
        Site {
            name: "Kermanshah",
            lat: 34.3142,
            lng: 47.0650,
        },
        Site {
            name: "Semnan",
            lat: 35.5769,
            lng: 53.3970,
        },
    ]
}

/// UAE target areas (population / infrastructure centres).
pub fn target_sites() -> Vec<Site> {
    vec![
        Site {
            name: "Abu Dhabi",
            lat: 24.4539,
            lng: 54.3773,
        },
        Site {
            name: "Dubai",
            lat: 25.2048,
            lng: 55.2708,
        },
        Site {
            name: "Al Dhafra Air Base",
            lat: 24.2483,
            lng: 54.5475,
        },
        Site {
            name: "Fujairah",
            lat: 25.1288,
            lng: 56.3264,
        },
        Site {
            name: "Jebel Ali",
            lat: 24.9857,
            lng: 55.0272,
        },
    ]
}
