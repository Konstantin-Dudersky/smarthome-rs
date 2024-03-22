use leptos::*;

use super::nav::Nav;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Nav />
      <main class="container"></main>
      <script type="text/javascript" src="tw-elements.umd.min.js"></script>
    }
}
