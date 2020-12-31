use dialoguer::{theme::TestTheme, Select};

use enigo::{Enigo, Key, KeyboardControllable};

#[test]
fn basic_navigation_produces_correct_selection() {
    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let mut enigo = Enigo::new();
    enigo.key_down(Key::Layout('j'));
    enigo.key_down(Key::Return);

    let selection = Select::with_theme(&TestTheme::default())
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    assert_eq!(Some(1), selection);
}
