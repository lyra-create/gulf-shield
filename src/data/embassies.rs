/// Embassy / consulate contact for a country represented in the UAE.
#[derive(Clone, Debug)]
pub struct Embassy {
    pub country: &'static str,
    pub phone: &'static str,
    pub emergency: &'static str,
    pub website: &'static str,
}

pub fn embassies() -> Vec<Embassy> {
    vec![
        Embassy {
            country: "United Kingdom",
            phone: "+971 2 610 1100",
            emergency: "+44 20 7008 5000",
            website: "https://www.gov.uk/world/organisations/british-embassy-abu-dhabi",
        },
        Embassy {
            country: "United States",
            phone: "+971 2 414 2200",
            emergency: "+1 202 501 4444",
            website: "https://ae.usembassy.gov/",
        },
        Embassy {
            country: "India",
            phone: "+971 2 449 2700",
            emergency: "+91 11 2301 2000",
            website: "https://www.indembassyuae.gov.in/",
        },
        Embassy {
            country: "Pakistan",
            phone: "+971 2 444 7800",
            emergency: "+92 51 920 7224",
            website: "https://pakembassyuae.org/",
        },
        Embassy {
            country: "Philippines",
            phone: "+971 2 634 0868",
            emergency: "+63 2 834 4000",
            website: "https://abudhabi.philembassy.net/",
        },
        Embassy {
            country: "Australia",
            phone: "+971 2 401 7500",
            emergency: "+61 2 6261 3305",
            website: "https://uae.embassy.gov.au/",
        },
        Embassy {
            country: "Canada",
            phone: "+971 2 694 0300",
            emergency: "+1 613 996 8885",
            website: "https://www.international.gc.ca/country-pays/uae-eau/",
        },
        Embassy {
            country: "France",
            phone: "+971 2 813 1000",
            emergency: "+33 1 43 17 53 53",
            website: "https://ae.ambafrance.org/",
        },
        Embassy {
            country: "Germany",
            phone: "+971 2 596 7700",
            emergency: "+49 30 1817 0",
            website: "https://abu-dhabi.diplo.de/",
        },
    ]
}
