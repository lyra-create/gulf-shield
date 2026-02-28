use leptos::prelude::*;

use crate::components::embassy::EmbassyDirectory;
use crate::components::info::InfoFooter;
use crate::components::map::MapView;
use crate::components::preparation::PreparationGuide;
use crate::components::safety::SafetyPanel;

/// Root application component.
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app-container">
            <header class="header">
                <h1>
                    "\u{1F6E1}\u{FE0F} Gulf Shield"
                </h1>
                <span class="subtitle">"Preparedness Information for the Gulf Region"</span>
            </header>

            <div class="main-content">
                <MapView />
                <aside class="sidebar">
                    <SafetyPanel />
                    <PreparationGuide />
                    <EmbassyDirectory />
                </aside>
            </div>

            <InfoFooter />
        </div>
    }
}
