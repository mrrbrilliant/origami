use origami_ui::MyButton;
use sycamore::prelude::*;
fn main() {
    sycamore::render(|| {
        template! {
            MyButton("What is your Name".to_string())
        }
    });
}
