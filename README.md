# YMX - Component Integration System

A lightweight YAML-based component integration system that allows you to define and execute components with property substitution and processing contexts.

## Features

- 🚀 Simple YAML-based component definitions
- 🔄 Property substitution with `$variable` and `${expression}` syntax
- 📦 Component calls with property passing
- 🎯 Command-line interface for easy integration
- ⚡ Fast and lightweight implementation

## Installation

### From Source

```bash
git clone <repository-url>
cd htymlx
cargo build --release
```

The compiled binary will be available at `target/release/ymx`.

## Usage

### Basic Usage

```bash
ymx <component_name> <yaml_file>
```

### With Properties

```bash
ymx my_component components.yml --property name=World --property count=42
```

### Verbose Output

```bash
ymx my_component components.yml --verbose
```

### Command Line Options

**Positional Arguments:**
- `<CALLER>`: The component name to call (required)
- `<FILE>`: YAML file containing components (required)

**Options:**
- `-p, --property <PROPERTY>`: Properties to pass to the component (format: key=value)
- `-v, --verbose`: Enable verbose output
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Component Definition

Components are defined in YAML files with the following structure:

### Basic Component

```yaml
# Simple literal component
hello: Hello World!

# Component with property substitution
greeting: Hello $name!

# Component with processing context
calculation: ${1 + 2}
```

### Component with Properties

```yaml
user_profile:
  name: $name
  age: $age
  email: "${name}@example.com"
```

### Component Calls

```yaml
nested_call:
  from!: base_component
  name: $user_name
  config:
    enabled: true
    timeout: 30
```

## Property Substitution

YMX supports two types of property substitution:

### Simple Variables

```yaml
message: Hello $name!
```

### Processing Contexts

```yaml
result: ${variable1 + variable2}
```

### Component References

```yaml
user_ref: $user
```

## Examples

### Example 1: Simple Greeting

Create a file `greeting.yml`:

```yaml
hello: Hello $name!
goodbye: Goodbye $name!
```

Run:

```bash
ymx hello greeting.yml --property name=Alice
# Output: Hello $name!

# Note: Property substitution needs to be implemented in the component execution
```

### Example 2: Component Calls

Create a file `components.yml`:

```yaml
base_button:
  label: $text
  style: primary

submit_button:
  from!: base_button
  text: Submit

cancel_button:
  from!: base_button
  text: Cancel
```

Run:

```bash
ymx submit_button components.yml --property text=Submit
# Output: Called component: base_button with params: {"text": "Submit"}
```

### Example 3: Processing Context

Create a file `math.yml`:

```yaml
add: ${a + b}
multiply: ${a * b}
```

Run:

```bash
ymx add math.yml --property a=5 --property b=3
# Output: Evaluated: ${a + b}
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running in Development Mode

```bash
cargo run -- component test.yml --property key=value
```

## Project Structure

```
htymlx/
├── src/
│   ├── lib.rs          # Core library implementation
│   └── main.rs         # CLI interface
├── tests/              # Test files
├── examples/           # Example YAML files
├── specs/              # Specifications and documentation
├── Cargo.toml          # Rust project configuration
└── README.md           # This file
```

## API Reference

### Core Types

```rust
pub enum ComponentValue {
    Literal(String),
    PropertyReference(String),
    ProcessingContext(String),
    ComponentCall(ComponentCall),
    Template(String),
}

pub struct ComponentCall {
    pub target: String,
    pub properties: HashMap<String, ComponentValue>,
}

pub struct YMXComponent {
    pub id: String,
    pub name: String,
    pub value: ComponentValue,
}
```

### Main Functions

```rust
// Parse YAML content into components
pub fn parse_yaml_content(content: &str) -> Result<Vec<YMXComponent>, String>

// Execute a component with given context
pub fn execute_component(
    component: &YMXComponent,
    context: &HashMap<String, String>
) -> Result<String, String>
```

## Configuration

The project can be configured through:

- **Environment variables**: For future extensions
- **Configuration files**: YAML-based configuration (planned)
- **Command-line arguments**: Current primary interface

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Enhanced property substitution
- [ ] Nested component execution
- [ ] Configuration file support
- [ ] Plugin system
- [ ] Web interface
- [ ] Component marketplace

## Troubleshooting

### Common Issues

1. **Component not found**: Check the component name spelling and ensure it exists in the YAML file
2. **Property not found**: Verify property names and use the `--verbose` flag to see available components
3. **YAML parsing error**: Validate your YAML syntax using online validators
4. **File not found**: Ensure the file path is correct and the file exists

### Debug Mode

Use the `--verbose` flag to get detailed information about:
- Which component is being called
- Properties being passed
- File being processed

## Support

For support and questions:

- Create an issue on the GitHub repository
- Check the [examples/](examples/) directory for sample configurations
- Review the [specs/](specs/) directory for detailed specifications

---

**YMX** - Making component integration simple and elegant.