fn main() {
    glib_build_tools::compile_resources(
        &["data/resources"],
        "data/resources/resources.gresource.xml",
        "simple_package_tracker.gresource",
    )
}
