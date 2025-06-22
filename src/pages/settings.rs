// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;

use crate::components::settings::{
    export_data::ExportDataCardComponent, export_pdf_options::ExportPDFOptionsComponent,
    import_data::ImportDataCardComponent, refresh_cache::RefreshCacheComponent,
    test_pdf_options::TestPDFOptionsComponent,
};

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="py-4 px-4 xl:px-96 flex flex-col gap-2">
            <h1 class="text-4xl font-bold">"Settings"</h1>

            <div>
                <h2 class="text-2xl font-bold">"Data"</h2>
                <div class="flex flex-wrap gap-2">
                    <ExportDataCardComponent/>
                    <ImportDataCardComponent/>
                </div>
            </div>
            <div>
                <h2 class="text-2xl font-bold">"Export PDF"</h2>
                <div class="flex flex-wrap gap-2">
                    <ExportPDFOptionsComponent/>
                    <TestPDFOptionsComponent/>
                </div>
            </div>
            <div>
                <h2 class="text-2xl font-bold">"Miscellaneous"</h2>
                <div class="flex flex-wrap gap-2">
                    <RefreshCacheComponent/>
                </div>
            </div>
            <br/>
            <p class="text-center">"Made by Alex Marín - FLOSS, now and always! - "<a class="link-primary" href="https://github.com/mariinkys/cocookies/releases" target="_blank">"v"{env!("CARGO_PKG_VERSION")}</a></p>
        </div>
    }
}
