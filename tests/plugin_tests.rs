use flowwire_plugins::manager::PluginManager;

#[test]
fn test_plugin_manager_creation() {
    let manager = PluginManager::new();

    assert_eq!(
        std::mem::size_of_val(&manager) > 0,
        true
    );
}

#[test]
fn test_plugin_tick_without_plugins() {
    let manager = PluginManager::new();

    assert!(manager.tick().is_ok());
}

#[test]
fn test_plugin_shutdown_without_plugins() {
    let manager = PluginManager::new();

    assert!(manager.shutdown().is_ok());
}
