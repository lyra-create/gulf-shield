use leptos::prelude::*;

/// Information footer shown at the bottom of the page.
#[component]
pub fn InfoFooter() -> impl IntoView {
    view! {
        <footer class="info-footer">
            "Data is approximate and for informational purposes only. \u{2022} "
            "Follow official "
            <a href="https://www.ncema.gov.ae" target="_blank" rel="noopener">"NCEMA"</a>
            " guidance. \u{2022} "
            "This tool does not predict attacks \u{2014} it shows potential scenarios to help you prepare. \u{2022} "
            <a href="https://www.ncema.gov.ae/en/app.aspx" target="_blank" rel="noopener">"Download NCEMA App"</a>
        </footer>
    }
}
