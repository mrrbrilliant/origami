use ::sycamore::prelude::*;

#[component(MyButton<G>)]
pub fn my_button(name: String) -> Template<G> {
    template! {
        button(class="bg-blue-500 text-white p-4 rounded-xl hover:shadow-2xl") {
            (name)
        }
    }
}
