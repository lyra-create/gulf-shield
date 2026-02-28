use leptos::prelude::*;

/// Safety-procedures panel shown in the sidebar.
#[component]
pub fn SafetyPanel() -> impl IntoView {
    view! {
        <div class="safety-section">
            <h2>"Safety Procedures"</h2>

            <h3>"During an alert"</h3>
            <ul class="safety-list">
                <li>"Get indoors immediately"</li>
                <li>"Stay away from windows and exterior walls"</li>
                <li>"Move to the lowest floor available"</li>
                <li>"Shelter in an interior room or corridor"</li>
                <li>"Keep a charged phone with you"</li>
                <li>"Stay tuned to official NCEMA channels"</li>
            </ul>

            <h3>"UAE Emergency Numbers"</h3>
            <div class="emergency-numbers">
                <div class="emergency-number">
                    <strong>"999"</strong>
                    <div>"Police"</div>
                </div>
                <div class="emergency-number">
                    <strong>"998"</strong>
                    <div>"Ambulance"</div>
                </div>
                <div class="emergency-number">
                    <strong>"997"</strong>
                    <div>"Civil Defence"</div>
                </div>
                <div class="emergency-number">
                    <strong>"911"</strong>
                    <div>"General"</div>
                </div>
            </div>

            <h3>"NCEMA Guidance"</h3>
            <p style="font-size:0.85rem; margin-top:0.3rem;">
                "The National Emergency Crisis and Disasters Management Authority "
                "provides official shelter and evacuation guidance."
            </p>
            <a class="ncema-link" href="https://www.ncema.gov.ae" target="_blank" rel="noopener">
                "Visit NCEMA \u{2192}"
            </a>

            <h3>"Shelter Locations"</h3>
            <p style="font-size:0.85rem; margin-top:0.3rem;">
                "The UAE has designated emergency shelters in major malls, "
                "metro stations, and public buildings. Follow NCEMA instructions "
                "for your nearest shelter location."
            </p>
        </div>
    }
}
