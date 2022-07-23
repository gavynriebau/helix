use anyhow::Result;
use crossterm::event::Event;
use helix_loader::plugin_dir;
use plugin::Plugin;

mod events;
mod exported_types;
mod plugin;

pub struct PluginManager {
    plugins: Vec<Plugin>,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    pub fn load_plugins(&mut self) -> Result<()> {
        log::info!("Loading plugins");

        let plugin_dir = plugin_dir();
        std::fs::create_dir_all(plugin_dir.clone())?;

        let entries = plugin_dir.read_dir()?;
        for entry in entries {
            match Plugin::try_from(entry) {
                Ok(plugin) => {
                    log::info!("Loaded plugin '{}'", plugin.name);
                    self.plugins.push(plugin);
                }
                Err(e) => log::error!("Failed to load plugin at entry: {}", e),
            }
        }

        log::info!("Finished loading plugins");

        Ok(())
    }

    pub fn start_plugins(&mut self) {
        log::info!("Starting plugins");
        for plugin in &mut self.plugins {
            if let Err(e) = plugin.start() {
                log::error!("Failed to start plugin '{}': {}", plugin.name, e);
            }
        }
        log::info!("Finished starting plugins");
    }

    pub fn handle_term_event(&mut self, term_event: Event) {
        for plugin in &mut self.plugins {
            let result = match term_event {
                Event::Key(event) => plugin.on_key_press(event),
                Event::Mouse(event) => plugin.on_mouse_event(event),
                Event::Resize(cols, rows) => plugin.on_resize(cols, rows),
            };

            if let Err(e) = result {
                log::error!(
                    "Plugin '{}' failed to handle term event: {}",
                    plugin.name,
                    e
                )
            }
        }
    }
}
