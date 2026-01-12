// JavaScript wrapper for YMX WASM module
// This file provides the web interface for YMX functionality

// Import the WASM module
import wasm_init from './wasm/ymx_wasm.js';

let wasmModule = null;

// Initialize the WASM module
export async function initialize() {
    if (!wasmModule) {
        wasmModule = await wasm_init('./wasm/ymx_wasm_bg.wasm');
    }
    return wasmModule;
}

// Process a YMX component
export async function process_component(yamlContent, componentName, properties = {}) {
    await initialize();
    
    // Convert properties to the format expected by WASM
    const propsArray = Object.entries(properties).flat();
    
    // Call the WASM function
    const result = wasmModule.process_component_yaml(
        yamlContent,
        componentName,
        propsArray
    );
    
    return result;
}

// Validate YAML syntax
export async function validate_yaml(yamlContent) {
    await initialize();
    
    try {
        wasmModule.validate_yaml_syntax(yamlContent);
        return { valid: true, error: null };
    } catch (error) {
        return { valid: false, error: error.message };
    }
}

// Get available components from YAML
export async function get_components(yamlContent) {
    await initialize();
    
    try {
        const components = wasmModule.parse_yaml_components(yamlContent);
        return JSON.parse(components);
    } catch (error) {
        return [];
    }
}