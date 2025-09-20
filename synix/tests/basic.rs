use synix::nix;

#[test]
fn basic_computation() {
    let x_plus_y = nix! {
        let
            x = 4;
            y = 5;
        in
            { x = x; inherit y; "test" = 5; }
    };
}
