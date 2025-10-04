use vital_reader::port::PortSelector;

#[test]
fn test_selector_module_exists() {
    // Just verify the module compiles and is accessible
    let _ = std::any::type_name::<PortSelector>();
}

#[test]
fn test_selector_get_port_with_cli_arg() {
    let port = PortSelector::get_port(Some("COM3")).unwrap();
    assert_eq!(port, "COM3");
}

#[test]
fn test_selector_methods_exist() {
    // Verify methods exist without calling interactive ones (they need stdin)
    let _ = std::any::type_name_of_val(&PortSelector::get_port);
    let _ = std::any::type_name_of_val(&PortSelector::select_interactive);
}