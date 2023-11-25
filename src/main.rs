use sycamore::prelude::*;

#[component]
fn App<G: Html>() -> View<G> {
  view! {
    main(class="flex-col flex-center") {
      div(class="flex-col flex-start center") {
        a(href="https://github.com/sycamore-rs/sycamore") {
          img(src="/asset/logo-sycamore.svg", alt="https://github.com/sycamore-rs/sycamore", size=100, height=100)
        }
        p() {
          "just a sycamore"
        }
      }
    }
  }
}

fn main() {
  sycamore::render(App);
}