use crate::events::{KeyEvent, MouseEvent};
use anyhow::{anyhow, Error, Result};
use rhai::module_resolvers::FileModuleResolver;
use rhai::packages::{Package, StandardPackage};
use rhai::plugin::*;
use rhai::{Dynamic, Engine, EvalAltResult, Scope, AST};
use std::fs::DirEntry;
use std::path::PathBuf;
use std::{ffi::OsStr, path::Path};

pub struct Plugin {
    pub name: String,
    pub engine: Engine,
    pub scope: Scope<'static>,
    pub ast: AST,
}

fn register_on_print(engine: &mut Engine, plugin_name: &str) {
    let plugin_name = plugin_name.to_string();
    engine.on_print(move |text| log::info!("Plugin ({}): {}", plugin_name, text));
}

fn register_on_debug(engine: &mut Engine, plugin_name: &str) {
    let plugin_name = plugin_name.to_string();
    engine.on_debug(move |text, source, pos| {
        if let Some(source) = source {
            log::debug!(
                "Plugin ({}): {} @ {:?} | {}",
                plugin_name,
                source,
                pos,
                text
            );
        } else if pos.is_none() {
            log::debug!("Plugin ({}): {}", plugin_name, text);
        } else {
            log::debug!("Plugin ({}): {:?} | {}", plugin_name, pos, text);
        }
    });
}

fn register_global_types_module(engine: &mut Engine) {
    let mut module = Module::new();

    combine_with_exported_module!(
        &mut module,
        "mouse_event_kind",
        crate::exported_types::mouse_event_kind
    );
    combine_with_exported_module!(
        &mut module,
        "mouse_event",
        crate::exported_types::mouse_event
    );
    combine_with_exported_module!(
        &mut module,
        "key_modifiers",
        crate::exported_types::key_modifiers
    );
    combine_with_exported_module!(&mut module, "key_event", crate::exported_types::key_event);

    engine.register_global_module(module.into());
}

fn get_plugin_name_and_script(entry: Result<DirEntry, std::io::Error>) -> Result<(String, String)> {
    let entry: DirEntry = entry?;

    let path: PathBuf = entry.path();
    let extension: &OsStr = path.extension().ok_or_else(|| {
        anyhow!(
            "Plugin at path '{}' did not have an extension",
            path.to_string_lossy()
        )
    })?;

    if extension != "rhai" {
        return Err(anyhow!(
            "Plugin at path '{}' did not have 'rhai' extension",
            path.to_string_lossy()
        ));
    }

    let file_name = entry.file_name();
    let file_stem = Path::new(&file_name)
        .file_stem()
        .ok_or_else(|| anyhow!("Failed to get file stem"))?
        .to_string_lossy();

    let name: String = String::from(file_stem);
    let script = std::fs::read_to_string(entry.path())?;

    Ok((name, script))
}

impl TryFrom<Result<DirEntry, std::io::Error>> for Plugin {
    type Error = Error;

    fn try_from(value: Result<DirEntry, std::io::Error>) -> Result<Self, Self::Error> {
        let (name, script) = get_plugin_name_and_script(value)?;

        let mut engine = Engine::new_raw();

        register_on_print(&mut engine, &name);
        register_on_debug(&mut engine, &name);

        engine.set_module_resolver(FileModuleResolver::new());

        let package = StandardPackage::new().as_shared_module();
        engine.register_global_module(package);

        register_global_types_module(&mut engine);

        let scope = Scope::new();
        let ast = engine.compile(script)?;

        Ok(Plugin {
            name,
            engine,
            scope,
            ast,
        })
    }
}

impl Plugin {
    pub fn start(&mut self) -> Result<()> {
        self.on_start()?;

        Ok(())
    }

    fn call_script_event_handler_fn(
        &mut self,
        name: &str,
        args: impl AsMut<[Dynamic]>,
    ) -> Result<(), anyhow::Error> {
        let result = self.engine.call_fn_raw(
            &mut self.scope,
            &self.ast,
            false, // Do not eval AST again as it gets evaluated when on_start is called
            true,  // Rewind scope
            name,
            None,
            args,
        );

        // If a plugin didn't define the function we ignore the error as callbacks are optional.
        return match result {
            Ok(_) => Ok(()),
            Err(e) => match *e {
                EvalAltResult::ErrorFunctionNotFound(_, _) => Ok(()),
                e => Err(anyhow!("Failed to call {}: {}", name, e)),
            },
        };
    }

    pub fn on_key_press(&mut self, key_press_event: crossterm::event::KeyEvent) -> Result<()> {
        let event = KeyEvent::from(key_press_event);
        self.call_script_event_handler_fn("on_key_press", [Dynamic::from(event)])?;

        Ok(())
    }

    pub fn on_mouse_event(&mut self, mouse_event: crossterm::event::MouseEvent) -> Result<()> {
        let event = MouseEvent::from(mouse_event);
        self.call_script_event_handler_fn("on_mouse_event", [Dynamic::from(event)])?;

        Ok(())
    }

    pub fn on_start(&mut self) -> Result<()> {
        self.engine
            .run_ast_with_scope(&mut self.scope, &self.ast)
            .map_err(|e| anyhow!("Failed to start plugin: {}", e))?;

        Ok(())
    }

    pub fn on_resize(&mut self, cols: u16, rows: u16) -> Result<()> {
        self.call_script_event_handler_fn(
            "on_resize",
            [(cols as i64).into(), (rows as i64).into()],
        )?;

        Ok(())
    }
}
