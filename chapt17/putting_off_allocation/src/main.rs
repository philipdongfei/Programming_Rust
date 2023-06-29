use std::borrow::Cow;

fn get_name() -> Cow<'static, str> {
    std::env::var("USER") // Windows uses "USERNAME"
        /*
        .map(|v| Cow::Owned(v)) // if succeeds
        .unwrap_or(Cow::Borrowed("whoever you are")) // if it fails
        */
        .map(|v| v.into())
        .unwrap_or("whoever you are".into())
}

fn get_title() -> Option<&'static str> {
    Some("jimb, Esq.")
}

fn main() {

    let mut name = get_name();
    if let Some(title) = get_title() {
        /*
        name.to_mut().push_str(", ");
        name.to_mut().push_str(title);
        */
        name += ", ";
        name += title;
    }
    println!("Greetings, {}!", name);

}
