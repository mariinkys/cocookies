use leptos::prelude::*;

// TODO: Fix dialog_title not being really optional.
#[component]
pub fn DialogComponent<S, D>(
    btn_text: S,
    btn_class: S,
    #[prop(optional)] dialog_title: D,
    #[prop(into)] dialog_content: ViewFn,
) -> impl IntoView
where
    S: ToString,
    D: ToString + Default,
{
    let dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();

    view! {
        <button
            class={btn_class.to_string()}
            on:click=move |_| {
                let _ = dialog_ref.get().unwrap().show_modal();
            }
        >
            {btn_text.to_string()}
        </button>

        <dialog class="modal" node_ref=dialog_ref>
          <div class="modal-box">
            <div class="flex justify-between items-center">
                <h2 class="text-2xl font-bold">{dialog_title.to_string()}</h2>
                <button
                    class="btn btn-sm btn-ghost"
                    on:click=move |_| {
                        dialog_ref.get().unwrap().close();
                    }
                >
                    "X"
                </button>
            </div>
            <br/>

            {dialog_content.run()}

          </div>
        </dialog>
    }
}
