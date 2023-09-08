fn main() {
    // env!
    let version = env!("CARGO_PKG_VERSION");

    println!("cargo pkg version:{}", &version);

    // include_str!
    /*
    const COMPOSITOR_SHADER: &str = 
        include_str!("../resources/compositor.glsl");
    println!("compositor shader: {}", COMPOSITOR_SHADER);
    */
    let my_str = include_str!("spanish.in");
    assert_eq!(my_str, "adiós\n");
    print!("{my_str}");

    // matches!
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));

}
