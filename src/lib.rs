uniffi::include_scaffolding!("template");

pub fn greeting(name: String) -> String {
    let mut message = String::from("Hello ");
    message.push_str(&name);
    message
}
