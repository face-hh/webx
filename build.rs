fn main() {
    glib_build_tools::compile_resources(
        &["src/resources"],
        "src/resources/icons.gresource.xml",
        "icons.gresource",
    );
}