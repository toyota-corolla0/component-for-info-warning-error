use leptos::*;

#[component]
pub fn NoteBlock(#[prop(into)] text: String) -> impl IntoView {
    view! {
        <div class="bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-4 mb-8 w-full" role="alert">
          <p class="font-bold">Note</p>
          <p>{ text }</p>
        </div>
    }
}

#[component]
pub fn SuccessBlock(#[prop(into)] text: String) -> impl IntoView {
    view! {
        <div class="bg-green-100 border-l-4 border-green-500 text-green-700 p-4 mb-8 w-full" role="alert">
            <p class="font-bold">Success</p>
            <p>{ text }</p>
        </div>
    }
}

#[component]
pub fn WarningBlock(#[prop(into)] text: String) -> impl IntoView {
    view! {
        <div class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4 mb-8 w-full" role="alert">
            <p class="font-bold">Warning</p>
            <p>{ text }</p>
        </div>
    }
}

#[component]
pub fn ErrorBlock(#[prop(into)] text: String) -> impl IntoView {
    view! {
        <div class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4 mb-8 w-full" role="alert">
            <p class="font-bold">Error</p>
            <p>{ text }</p>
        </div>
    }
}
