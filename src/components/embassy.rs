use leptos::prelude::*;

use crate::data::embassies::embassies;

/// Expandable embassy-directory section.
#[component]
pub fn EmbassyDirectory() -> impl IntoView {
    let (open, set_open) = signal(false);

    let toggle = move |_| set_open.update(|v| *v = !*v);

    let arrow_class = move || if open.get() { "arrow open" } else { "arrow" };

    view! {
        <div class="embassy-section">
            <h2>"Embassy Contacts"</h2>
            <p style="font-size:0.8rem; margin-bottom:0.5rem; color:#7f8c8d;">
                "Contact your embassy for country-specific evacuation advice."
            </p>
            <button class="embassy-toggle" on:click=toggle>
                <span>"Show embassy directory"</span>
                <span class=arrow_class>"\u{25B6}"</span>
            </button>

            <Show when=move || open.get()>
                <div class="embassy-list">
                    {embassies().into_iter().map(|e| {
                        view! {
                            <div class="embassy-card">
                                <div class="country">{e.country}</div>
                                <div class="phone">{format!("Tel: {}", e.phone)}</div>
                                <div class="phone">{format!("Emergency: {}", e.emergency)}</div>
                                <a href={e.website} target="_blank" rel="noopener">"Visit website \u{2192}"</a>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </Show>
        </div>
    }
}
