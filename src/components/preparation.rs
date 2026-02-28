use leptos::prelude::*;

/// Collapsible section helper.
#[component]
fn PrepSection(title: &'static str, icon: &'static str, children: Children) -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <div class="prep-section">
            <button
                class="prep-toggle"
                on:click=move |_| set_open.update(|v| *v = !*v)
            >
                <span class="prep-icon">{icon}</span>
                <span class="prep-title">{title}</span>
                <span class="arrow" class:open=open>"\u{25B6}"</span>
            </button>
            <div class="prep-content" class:visible=open>
                {children()}
            </div>
        </div>
    }
}

/// A single supply item with optional link.
#[component]
fn SupplyItem(
    name: &'static str,
    detail: &'static str,
    #[prop(optional)] link: Option<&'static str>,
    #[prop(optional)] link_label: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="supply-item">
            <div class="supply-name">{name}</div>
            <div class="supply-detail">{detail}</div>
            {link.map(|href| view! {
                <a class="supply-link" href=href target="_blank" rel="noopener">
                    {link_label.unwrap_or("Buy \u{2192}")}
                </a>
            })}
        </div>
    }
}

/// Preparation guide panel.
#[component]
pub fn PreparationGuide() -> impl IntoView {
    view! {
        <div class="preparation-section">
            <h2>"\u{1F4E6} Preparation"</h2>
            <p class="prep-intro">
                "Practical steps you can take now. Stock supplies "
                "before you need them, not during a crisis."
            </p>

            <PrepSection title="Water" icon="\u{1F4A7}">
                <p class="prep-note">
                    "Minimum 3.8 litres per person per day. "
                    "Stock at least 7 days. A family of four needs ~110 litres."
                </p>
                <SupplyItem
                    name="Large bottles (5L/6L packs)"
                    detail="Cheapest per litre. Store in a cool, dark place. Rotate every 6 months."
                    link="https://www.amazon.ae/s?k=drinking+water+5+litre&crid=1"
                    link_label="Amazon.ae \u{2192}"
                />
                <SupplyItem
                    name="Sealed 500ml cases"
                    detail="Grab-and-go if you need to evacuate. Keep a case in the car."
                    link="https://www.noon.com/uae-en/search?q=water+500ml+case"
                    link_label="Noon \u{2192}"
                />
                <SupplyItem
                    name="Water purification tablets"
                    detail="Backup option. Treats tap/stored water if supply is disrupted."
                    link="https://www.amazon.ae/s?k=water+purification+tablets"
                    link_label="Amazon.ae \u{2192}"
                />
                <div class="prep-tip">
                    <strong>"Tip:"</strong>
                    " Fill your bathtub during early warnings. "
                    "That's 150+ litres of non-drinking water for sanitation."
                </div>
            </PrepSection>

            <PrepSection title="Non-Perishable Food" icon="\u{1F96B}">
                <p class="prep-note">
                    "7 days minimum. Choose foods that need no cooking, "
                    "no refrigeration, and no water to prepare."
                </p>
                <SupplyItem
                    name="Canned food (beans, tuna, chickpeas)"
                    detail="High protein, long shelf life. Don't forget a manual can opener."
                    link="https://www.noon.com/uae-en/search?q=canned+food+tuna+beans"
                    link_label="Noon \u{2192}"
                />
                <SupplyItem
                    name="Dried food (dates, nuts, dried fruit)"
                    detail="Calorie-dense, lightweight. Dates are perfect for the Gulf climate."
                    link="https://www.amazon.ae/s?k=dried+dates+nuts+emergency"
                    link_label="Amazon.ae \u{2192}"
                />
                <SupplyItem
                    name="Crackers, biscuits, cereal bars"
                    detail="No prep needed. Keep variety for morale, especially with children."
                    link="https://www.carrefouruae.com/mafuae/en/v4/search?keyword=crackers+biscuits+bars"
                    link_label="Carrefour UAE \u{2192}"
                />
                <SupplyItem
                    name="Peanut butter / Nutella"
                    detail="High calorie, no refrigeration needed once sealed. Comfort food."
                    link="https://www.noon.com/uae-en/search?q=peanut+butter"
                    link_label="Noon \u{2192}"
                />
                <SupplyItem
                    name="UHT / powdered milk"
                    detail="Essential if you have children. Shelf-stable for months."
                    link="https://www.amazon.ae/s?k=uht+milk+long+life"
                    link_label="Amazon.ae \u{2192}"
                />
                <div class="prep-tip">
                    <strong>"Tip:"</strong>
                    " Don't buy food you wouldn't normally eat. Rotate stock "
                    "by eating it and replacing. Nothing should expire on the shelf."
                </div>
            </PrepSection>

            <PrepSection title="Emergency Kit" icon="\u{1FA79}">
                <p class="prep-note">
                    "One bag, ready to grab. Keep it near your front door or in the car."
                </p>
                <SupplyItem
                    name="First aid kit"
                    detail="Bandages, antiseptic, painkillers, any prescription meds (7-day supply)."
                    link="https://www.amazon.ae/s?k=first+aid+kit"
                    link_label="Amazon.ae \u{2192}"
                />
                <SupplyItem
                    name="Torch + spare batteries"
                    detail="Power cuts are likely. Get a hand-crank or solar option as backup."
                    link="https://www.amazon.ae/s?k=emergency+torch+flashlight+batteries"
                    link_label="Amazon.ae \u{2192}"
                />
                <SupplyItem
                    name="Power bank (20,000mAh+)"
                    detail="Keep your phone alive. Charge it fully and leave it in the kit."
                    link="https://www.noon.com/uae-en/search?q=power+bank+20000mah"
                    link_label="Noon \u{2192}"
                />
                <SupplyItem
                    name="Battery-powered or hand-crank radio"
                    detail="If mobile networks go down, FM radio is your lifeline for official updates."
                    link="https://www.amazon.ae/s?k=emergency+hand+crank+radio"
                    link_label="Amazon.ae \u{2192}"
                />
                <SupplyItem
                    name="Copies of important documents"
                    detail="Passport, Emirates ID, insurance, visa. Sealed in a waterproof bag."
                />
                <SupplyItem
                    name="Cash (small notes)"
                    detail="ATMs and card terminals may be offline. Keep AED 500-1000 in small bills."
                />
                <SupplyItem
                    name="Dust masks / N95"
                    detail="Debris, dust, smoke from interceptions. Protect your lungs."
                    link="https://www.amazon.ae/s?k=n95+mask"
                    link_label="Amazon.ae \u{2192}"
                />
            </PrepSection>

            <PrepSection title="Your Home" icon="\u{1F3E0}">
                <p class="prep-note">
                    "Simple changes that make a real difference during an attack."
                </p>
                <div class="supply-item">
                    <div class="supply-name">"Identify your safe room"</div>
                    <div class="supply-detail">
                        "Interior room, lowest floor, no windows. Bathroom or corridor works. "
                        "Everyone in the household should know which room."
                    </div>
                </div>
                <div class="supply-item">
                    <div class="supply-name">"Window film"</div>
                    <div class="supply-detail">
                        "Safety film stops glass from shattering into fragments. "
                        "Cheap and easy to apply. Prioritise bedrooms and living areas."
                    </div>
                </div>
                <div class="supply-item">
                    <div class="supply-name">"Know your utilities"</div>
                    <div class="supply-detail">
                        "Find your water main, gas valve, and electricity breaker. "
                        "Practice shutting them off. Label them clearly."
                    </div>
                </div>
                <div class="supply-item">
                    <div class="supply-name">"Family communication plan"</div>
                    <div class="supply-detail">
                        "Pick a meeting point outside your building. Choose an out-of-area contact "
                        "everyone can reach. Make sure children know the plan."
                    </div>
                </div>
                <div class="prep-tip">
                    <strong>"Tip:"</strong>
                    " Keep shoes and a torch next to every bed. If windows shatter "
                    "at night, you'll be walking through glass."
                </div>
            </PrepSection>

            <PrepSection title="Useful Apps & Links" icon="\u{1F4F1}">
                <div class="app-links">
                    <a class="app-link" href="https://www.ncema.gov.ae/en/app.aspx" target="_blank" rel="noopener">
                        <strong>"NCEMA App"</strong>
                        <span>"Official UAE emergency alerts"</span>
                    </a>
                    <a class="app-link" href="https://www.moi.gov.ae/en/eservices/moismartapp.aspx" target="_blank" rel="noopener">
                        <strong>"MOI UAE App"</strong>
                        <span>"Ministry of Interior services"</span>
                    </a>
                    <a class="app-link" href="https://www.dha.gov.ae/en/DHAApp" target="_blank" rel="noopener">
                        <strong>"DHA App"</strong>
                        <span>"Dubai Health Authority"</span>
                    </a>
                    <a class="app-link" href="https://u.ae/en/information-and-services/health-and-fitness/health-emergencies" target="_blank" rel="noopener">
                        <strong>"UAE Health Emergencies"</strong>
                        <span>"Government health emergency info"</span>
                    </a>
                    <a class="app-link" href="https://www.redcrescent.ae" target="_blank" rel="noopener">
                        <strong>"Emirates Red Crescent"</strong>
                        <span>"Humanitarian aid and shelters"</span>
                    </a>
                </div>
            </PrepSection>

        </div>
    }
}
