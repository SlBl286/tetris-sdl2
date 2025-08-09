// #![windows_subsystem = "windows"]
fn main() {
    embed_resource::compile("app.rc", embed_resource::NONE);
}