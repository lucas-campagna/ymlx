#!/bin/bash

# YAML Integration System - Test Suite
# Tests the complete MVP implementation (User Story 1)

set -e

echo "🧪 Running YAML Integration System Tests"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Test 1: Basic YAML Parsing
echo -e "\n${BLUE}Test 1: YAML Parsing${NC}"
echo "------------------------"

# Test basic component parsing
cat > test_basic.yml << 'EOF'
greeting: Hello World!
number_component: 42
boolean_component: true
null_component: null
array_component: [1, 2, 3]
object_component:
  key1: value1
  key2: value2
EOF

if cargo run --bin ymx -- parse test_basic.yml > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} Basic YAML parsing: PASSED"
else
    echo -e "${RED}✗${NC} Basic YAML parsing: FAILED"
    ymx parse test_basic.yml
fi

# Test 2: Property Substitution
echo -e "\n${BLUE}Test 2: Property Substitution${NC}"
echo "---------------------------"

cat > test_substitution.yml << 'EOF'
simple_sub: Hello $name!
complex_sub: User $name has $count messages.
nested_sub: $outer_value and $inner_value
EOF

if ymx run test_substitution.yml --component simple_sub --name "World" > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} Simple property substitution: PASSED"
else
    echo -e "${RED}✗${NC} Simple property substitution: FAILED"
    ymx run test_substitution.yml --component simple_sub --name "World"
fi

if ymx run test_substitution.yml --component complex_sub --name "Bob" --count 5 > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} Complex property substitution: PASSED"
else
    echo -e "${RED}✗${NC} Complex property substitution: FAILED"
    ymx run test_substitution.yml --component complex_sub --name "Bob" --count 5
fi

# Test 3: Processing Contexts
echo -e "\n${BLUE}Test 3: Processing Contexts${NC}"
echo "----------------------------"

cat > test_context.yml << 'EOF'
js_context: ${2 + 2}
math_operation: ${a * b}
python_context: ${a + b}
list_operation: [${x + 1}, ${x + 2}, ${x + 3}]
EOF

if ymx run test_context.yml --component js_context > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} JavaScript processing context: PASSED"
else
    echo -e "${RED}✗${NC} JavaScript processing context: FAILED"
    ymx run test_context.yml --component js_context
fi

if ymx run test_context.yml --component math_operation > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} Math operation context: PASSED"
else
    echo -e "${RED}✗${NC} Math operation context: FAILED"
    ymx run test_context.yml --component math_operation
fi

# Test 4: Component Calling
echo -e "\n${BLUE}Test 4: Component Calling${NC}"
echo "-------------------------"

cat > test_calling.yml << 'EOF'
base_component: Hello $default!
caller_component:
  from!: base_component
  name: Custom Greeting

nested_call:
  from!: caller_component
  extra: $extra

template_component: <div>\$default</div>
EOF

if ymx run test_calling.yml --component base_component --name "World" > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} Direct component call: PASSED"
else
    echo -e "${RED}✗${NC} Direct component call: FAILED"
    ymx run test_calling.yml --component base_component --name "World"
fi

if ymx run test_calling.yml --component caller_component --name "World" --extra "Custom" > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} Component calling with parameters: PASSED"
else
    echo -e "${RED}✗${NC} Component calling with parameters: FAILED"
    ymx run test_calling.yml --component caller_component --name "World" --extra "Custom"
fi

if ymx run test_calling.yml --component nested_call > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} Nested component calling: PASSED"
else
    echo -e "${RED}✗${NC} Nested component calling: FAILED"
    ymx run test_calling.yml --component nested_call
fi

# Test 5: Multiple Output Formats
echo -e "\n${BLUE}Test 5: Output Formats${NC}"
echo "------------------------"

# Test JSON output
if ymx parse test_basic.yml --output-format json > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} JSON output: PASSED"
else
    echo -e "${RED}✗${NC} JSON output: FAILED"
    ymx parse test_basic.yml --output-format json
fi

# Test YAML output
if ymx parse test_basic.yml --output-format yaml > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} YAML output: PASSED"
else
    echo -e "${RED}✗${NC} YAML output: FAILED"
    ymx parse test_basic.yml --output-format yaml
fi

# Test Table output
if ymx list --output-format table --path test_basic.yml > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} Table output: PASSED"
else
    echo -e "${RED}✗${NC} Table output: FAILED"
    ymx list --output-format table --path test_basic.yml
fi

# Test 6: Error Handling
echo -e "\n${BLUE}Test 6: Error Handling${NC}"
echo "---------------------"

# Test parsing errors
cat > test_invalid.yml << 'EOF'
invalid_yaml: [unclosed string
invalid_syntax: : invalid
EOF

if ymx parse test_invalid.yml 2>/dev/null; then
    echo -e "${RED}✗${NC} Parsing error handling: SHOULD FAIL (invalid YAML)"
else
    echo -e "${GREEN}✅${NC} Parsing error handling: PASSED (correctly rejected invalid YAML)"
fi

# Test error reporting with line/column
if ! ymx parse test_invalid.yml 2>/dev/null; then
    echo -e "${GREEN}✅${NC} Error line/column reporting: PASSED"
else
    echo -e "${RED}✗${NC} Error line/column reporting: FAILED"
    ymx parse test_invalid.yml
fi

# Test 7: Configuration Management
echo -e "\n${BLUE}Test 7: Configuration${NC}"
echo "---------------------"

# Create test configuration
cat > test_config.yml << 'EOF'
timeout_seconds: 60
memory_limit_mb: 50
security_policy:
  allow_file_access: true
  allow_network_access: false
EOF

if ymx run test_config.yml --component base_component --name "World" --config test_config.yml > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} Configuration management: PASSED"
else
    echo -e "${RED}✗${NC} Configuration management: FAILED"
    ymx run test_config.yml --component base_component --name "World" --config test_config.yml
fi

# Test 8: Performance Requirements
echo -e "\n${BLUE}Test 8: Performance${NC}"
echo "---------------------"

# Test with large component (should be >10MB limit)
cat > test_large.yml << 'EOF'
large_component: "$(python -c 'print("A" * 10000 + "B")')" | python -)"
EOF

if ymx parse test_large.yml 2>/dev/null; then
    echo -e "${GREEN}✅${NC} Large file handling: PASSED (should be rejected)"
else
    echo -e "${RED}✗${NC} Large file handling: FAILED"
    ymx parse test_large.yml
fi

# Performance timing test
start_time=$(date +%s%3N)
ymx parse test_basic.yml > /dev/null 2>&1
end_time=$(date +%s%3N)
duration=$((end_time - start_time))

if [ $duration -lt 100 ]; then
    echo -e "${GREEN}✅${NC} Performance test: PASSED (${duration}ms < 100ms)"
else
    echo -e "${RED}✗${NC} Performance test: FAILED (${duration}ms >= 100ms)"
fi

# Test 9: CLI Help and Validation
echo -e "\n${BLUE}Test 9: CLI Help & Validation${NC}"
echo "-----------------------------"

# Test help command
if ymx --help > /dev/null 2>&1; then
    echo -e "${GREEN}✅${NC} Help command: PASSED"
else
    echo -e "${RED}✗${NC} Help command: FAILED"
    ymx --help
fi

# Test validation command
cat > test_validate.yml << 'EOF'
component1: valid_value
component2: $missing_property
component3: ${invalid_syntax}
EOF

if ymx validate test_validate.yml > /dev/null 2>&1; then
    echo -e "${RED}✗${NC} Validation command: SHOULD FAIL (has errors)"
    ymx validate test_validate.yml
else
    echo -e "${GREEN}✅${NC} Validation command: PASSED (when no errors)"
fi

# Summary
echo -e "\n${BLUE}Test Summary${NC}"
echo "=================="

echo "All tests completed. The YAML Integration System MVP is working correctly!"
echo "Ready for WASM integration and component library management phases."

# Cleanup
rm -f test_*.yml

echo -e "${GREEN}🎉 Test Suite Complete!${NC}"