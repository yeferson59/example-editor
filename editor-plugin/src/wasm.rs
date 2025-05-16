//! WebAssembly plugin implementation

use std::path::Path;
use std::sync::Mutex;
use wasmer::{Store, Module, Instance, Value, MemoryAccessError, imports};
use crate::{Plugin, PluginConfig, PluginMetadata, Result, PluginError};

// Add error conversions for wasmer errors
impl From<wasmer::ExportError> for PluginError {
    fn from(err: wasmer::ExportError) -> Self {
        PluginError::ExecutionError(err.to_string())
    }
}

impl From<wasmer::RuntimeError> for PluginError {
    fn from(err: wasmer::RuntimeError) -> Self {
        PluginError::ExecutionError(err.to_string())
    }
}

impl From<MemoryAccessError> for PluginError {
    fn from(err: MemoryAccessError) -> Self {
        PluginError::ExecutionError(format!("Memory access error: {}", err))
    }
}

impl From<anyhow::Error> for PluginError {
    fn from(err: anyhow::Error) -> Self {
        PluginError::ExecutionError(err.to_string())
    }
}

impl From<serde_json::Error> for PluginError {
    fn from(err: serde_json::Error) -> Self {
        PluginError::ExecutionError(err.to_string())
    }
}

/// WebAssembly plugin
pub struct WasmPlugin {
    /// Plugin instance
    instance: Instance,
    /// Plugin metadata
    metadata: PluginMetadata,
    /// WebAssembly store with interior mutability
    store: Mutex<Store>,
}

impl WasmPlugin {
    #[allow(dead_code)]
    /// Allocates a string in WebAssembly memory
    fn allocate_string(&mut self, s: &str) -> Result<i32> {
        // Get the allocate function
        let allocate = self.instance.exports.get_function("allocate")
            .map_err(|e| PluginError::ExecutionError(format!("Missing allocate function: {}", e)))?;
        
        // Allocate memory for the string
        let len = s.len() as i32;
        let mut store = self.store.lock().unwrap();
        let result = allocate.call(&mut store, &[wasmer::Value::I32(len)])
            .map_err(|e| PluginError::ExecutionError(e.to_string()))?;
        
        let ptr = result[0].unwrap_i32();
        
        // Write the string to memory
        let memory = self.instance.exports.get_memory("memory")
            .map_err(|e| PluginError::ExecutionError(format!("Missing memory export: {}", e)))?;
        
        let view = memory.view(&store);
        view.write(ptr as u64, s.as_bytes())
            .map_err(|e| PluginError::ExecutionError(format!("Failed to write memory: {}", e)))?;
        
        Ok(ptr)
    }
    
    #[allow(dead_code)]
    /// Reads a string from WebAssembly memory
    fn read_string(&self, ptr: i32, len: i32) -> Result<String> {
        let memory = self.instance.exports.get_memory("memory")
            .map_err(|e| PluginError::ExecutionError(format!("Missing memory export: {}", e)))?;
        
        let store = self.store.lock().unwrap();
        let view = memory.view(&store);
        let mut buffer = vec![0u8; len as usize];
        
        view.read(ptr as u64, &mut buffer)
            .map_err(|e| PluginError::ExecutionError(format!("Failed to read memory: {}", e)))?;
        
        String::from_utf8(buffer)
            .map_err(|e| PluginError::ExecutionError(format!("Invalid UTF-8: {}", e)))
    }

    /// Allocates memory in the WebAssembly instance
    fn alloc(&self, size: usize) -> Result<u32> {
        let alloc = self.instance
            .exports
            .get_function("alloc")?;

        let mut store = self.store.lock().unwrap();
        let result = alloc.call(&mut store, &[Value::I32(size as i32)])?;
        
        Ok(result[0].unwrap_i32() as u32)
    }

    /// Writes data to WebAssembly memory
    fn write_memory(&self, ptr: u32, data: &[u8]) -> Result<()> {
        let memory = self.instance.exports.get_memory("memory")?;
        let store = self.store.lock().unwrap();
        let view = memory.view(&store);
        let offset = ptr as u64;
        
        if offset + data.len() as u64 > view.data_size() {
            return Err(PluginError::ExecutionError("Memory out of bounds".to_string()));
        }
        
        view.write(offset, data)?;
        Ok(())
    }

    /// Reads data from WebAssembly memory
    fn read_memory(&self, ptr: u32) -> Result<Vec<u8>> {
        let memory = self.instance.exports.get_memory("memory")?;
        let store = self.store.lock().unwrap();
        let view = memory.view(&store);
        
        // Read the length prefix (assuming 4-byte length prefix)
        let len_offset = ptr as u64;
        if len_offset + 4 > view.data_size() {
            return Err(PluginError::ExecutionError("Memory out of bounds when reading length".to_string()));
        }
        
        let mut len_bytes = [0u8; 4];
        view.read(len_offset, &mut len_bytes)?;
        let len = u32::from_le_bytes(len_bytes) as usize;
        
        // Read the actual data
        let data_offset = len_offset + 4;
        if data_offset + len as u64 > view.data_size() {
            return Err(PluginError::ExecutionError("Memory out of bounds when reading data".to_string()));
        }
        
        let mut data = vec![0u8; len];
        view.read(data_offset, &mut data)?;
        
        Ok(data)
    }

    /// Loads a WebAssembly plugin from a path
    pub async fn load(path: impl AsRef<Path>, config: PluginConfig) -> Result<Self> {
        let path = path.as_ref();
        let wasm_path = path.join(&config.manifest.entry_point).with_extension("wasm");

        let wasm_bytes = std::fs::read(&wasm_path)?;

        // Create a store
        let mut store = Store::default();

        // Compile the WebAssembly module
        let module = Module::new(&store, &wasm_bytes)
            .map_err(|e| PluginError::LoadError(e.to_string()))?;

        // Create import object
        let import_object = imports! {};

        // Instantiate the module
        let instance = Instance::new(&mut store, &module, &import_object)
            .map_err(|e| PluginError::LoadError(e.to_string()))?;

        Ok(Self {
            instance,
            metadata: PluginMetadata {
                name: config.manifest.name,
                version: config.manifest.version,
                description: config.manifest.description,
            },
            store: Mutex::new(store),
        })
    }

    /// Calls a WebAssembly function
    fn call_wasm_function(&self, name: &str, args: &[Value]) -> Result<Vec<Value>> {
        let function = self.instance.exports.get_function(name)?;
        let mut store = self.store.lock().unwrap();
        let result = function.call(&mut store, args)?;
        Ok(result.into_vec())
    }
}

#[async_trait::async_trait]
impl Plugin for WasmPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    async fn initialize(&mut self) -> Result<()> {
        self.call_wasm_function("initialize", &[])?;
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        self.call_wasm_function("shutdown", &[])?;
        Ok(())
    }

    async fn execute(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        // Convert the command and args to bytes that can be passed to WASM
        let command_bytes = command.as_bytes();
        let args_bytes = serde_json::to_vec(&args)?;
        
        // Get memory for the command
        let command_ptr = self.alloc(command_bytes.len())?;
        self.write_memory(command_ptr, &command_bytes)?;
        
        // Get memory for the args
        let args_ptr = self.alloc(args_bytes.len())?;
        self.write_memory(args_ptr, &args_bytes)?;
        
        // Call the execute function
        let result = self.call_wasm_function(
            "execute",
            &[Value::I32(command_ptr as i32), Value::I32(args_ptr as i32)]
        )?;
        
        let result_ptr = result[0].unwrap_i32() as u32;
            
        // Read the result
        let result_bytes = self.read_memory(result_ptr)?;
        let result: serde_json::Value = serde_json::from_slice(&result_bytes)?;
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: This test requires a real WebAssembly plugin to work
    // #[tokio::test]
    // async fn test_wasm_plugin_loading() {
    //     // Test implementation
    // }
}
