#!/bin/bash

# Comprehensive test runner for YMX project
# Runs all test suites and generates a comprehensive report

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test categories
TEST_CATEGORIES=(
    "comprehensive_parsers"
    "component_execution"
    "property_substitution"
    "edge_cases"
    "performance_tests"
    "security_tests"
    "integration_tests"
)

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}=== $1 ===${NC}"
}

# Function to run a specific test category
run_test_category() {
    local category=$1
    print_header "Running $category Tests"
    
    if cargo test --test $category --lib -- --nocapture; then
        print_status "$category tests passed"
        return 0
    else
        print_error "$category tests failed"
        return 1
    fi
}

# Function to run WASM tests
run_wasm_tests() {
    print_header "Running WASM Tests"
    
    if command -v wasm-pack >/dev/null 2>&1; then
        # Build WASM for testing
        wasm-pack build --target web --out-dir pkg --release --dev
        
        # Run WASM tests if wasm-bindgen-test is available
        if command -v wasm-bindgen-test-runner >/dev/null 2>&1; then
            if wasm-pack test --node; then
                print_status "WASM tests passed"
                return 0
            else
                print_error "WASM tests failed"
                return 1
            fi
        else
            print_warning "wasm-bindgen-test-runner not found, skipping WASM tests"
            return 0
        fi
    else
        print_warning "wasm-pack not found, skipping WASM tests"
        return 0
    fi
}

# Function to run CLI integration tests
run_cli_tests() {
    print_header "Running CLI Integration Tests"
    
    # Test basic CLI functionality
    if cargo run -- --help >/dev/null 2>&1; then
        print_status "CLI help works"
    else
        print_error "CLI help failed"
        return 1
    fi
    
    # Test CLI version
    if cargo run -- --version >/dev/null 2>&1; then
        print_status "CLI version works"
    else
        print_error "CLI version failed"
        return 1
    fi
    
    # Test CLI with sample file
    if cargo run -- component test_simple.yml --property name=Test >/dev/null 2>&1; then
        print_status "CLI basic execution works"
    else
        print_error "CLI basic execution failed"
        return 1
    fi
    
    return 0
}

# Function to generate test report
generate_report() {
    local report_file="test_report_$(date +%Y%m%d_%H%M%S).txt"
    
    print_header "Generating Test Report"
    
    cat > "$report_file" << EOF
YMX Test Report
===============
Date: $(date)
Platform: $(uname -a)
Rust Version: $(rustc --version)

Test Categories:
EOF
    
    for category in "${TEST_CATEGORIES[@]}"; do
        echo "- $category" >> "$report_file"
    done
    
    cat >> "$report_file" << EOF

Test Execution Summary:
- Comprehensive Parser Tests: Core YAML parsing functionality
- Component Execution Tests: Component execution and rendering
- Property Substitution Tests: Variable replacement and expression evaluation
- Edge Cases Tests: Boundary conditions and unusual inputs
- Performance Tests: Benchmarks and scalability tests
- Security Tests: Injection attempts and vulnerability checks
- Integration Tests: End-to-end functionality and CLI testing
- WASM Tests: WebAssembly compatibility and browser functionality

Coverage Areas:
- YAML parsing and validation
- Component type detection and handling
- Property reference resolution
- Expression evaluation and processing contexts
- Component calls and inheritance
- Error handling and edge cases
- Performance and memory management
- Security vulnerability prevention
- CLI interface functionality
- WebAssembly compatibility

Test Environment:
- Compiler: $(rustc --version)
- Target: $(rustc -vV | grep 'host:' | cut -d' ' -f2)
- Features: All default features enabled
- Optimization: Debug builds for tests, Release for performance tests

EOF
    
    print_status "Test report generated: $report_file"
}

# Function to run coverage analysis
run_coverage() {
    print_header "Running Code Coverage Analysis"
    
    if command -v cargo-tarpaulin >/dev/null 2>&1; then
        if cargo tarpaulin --out Html --output-dir target/coverage; then
            print_status "Coverage report generated in target/coverage/"
            return 0
        else
            print_warning "Coverage analysis failed, continuing without it"
            return 0
        fi
    else
        print_warning "cargo-tarpaulin not found, skipping coverage analysis"
        return 0
    fi
}

# Function to run benchmarks
run_benchmarks() {
    print_header "Running Performance Benchmarks"
    
    if cargo test --release --test performance_tests -- --nocapture; then
        print_status "Benchmarks completed successfully"
        return 0
    else
        print_error "Benchmarks failed"
        return 1
    fi
}

# Function to run security audit
run_security_audit() {
    print_header "Running Security Audit"
    
    if command -v cargo-audit >/dev/null 2>&1; then
        if cargo audit; then
            print_status "Security audit passed"
            return 0
        else
            print_error "Security audit found vulnerabilities"
            return 1
        fi
    else
        print_warning "cargo-audit not found, skipping security audit"
        return 0
    fi
}

# Main execution logic
main() {
    local failed_tests=0
    local total_tests=0
    
    print_header "YMX Comprehensive Test Suite"
    echo "Starting comprehensive test execution..."
    echo
    
    # Run all test categories
    for category in "${TEST_CATEGORIES[@]}"; do
        total_tests=$((total_tests + 1))
        if ! run_test_category "$category"; then
            failed_tests=$((failed_tests + 1))
        fi
        echo
    done
    
    # Run WASM tests
    total_tests=$((total_tests + 1))
    if ! run_wasm_tests; then
        failed_tests=$((failed_tests + 1))
    fi
    echo
    
    # Run CLI integration tests
    total_tests=$((total_tests + 1))
    if ! run_cli_tests; then
        failed_tests=$((failed_tests + 1))
    fi
    echo
    
    # Run coverage analysis (optional)
    if command -v cargo-tarpaulin >/dev/null 2>&1; then
        if [ "$1" = "--coverage" ] || [ "$1" = "--all" ]; then
            total_tests=$((total_tests + 1))
            if ! run_coverage; then
                failed_tests=$((failed_tests + 1))
            fi
            echo
        fi
    fi
    
    # Run benchmarks (optional)
    if [ "$1" = "--bench" ] || [ "$1" = "--all" ]; then
        total_tests=$((total_tests + 1))
        if ! run_benchmarks; then
            failed_tests=$((failed_tests + 1))
        fi
        echo
    fi
    
    # Run security audit (optional)
    if [ "$1" = "--security" ] || [ "$1" = "--all" ]; then
        total_tests=$((total_tests + 1))
        if ! run_security_audit; then
            failed_tests=$((failed_tests + 1))
        fi
        echo
    fi
    
    # Generate final report
    generate_report
    
    # Print summary
    print_header "Test Execution Summary"
    echo "Total test categories: $total_tests"
    echo "Failed test categories: $failed_tests"
    echo "Passed test categories: $((total_tests - failed_tests))"
    
    if [ $failed_tests -eq 0 ]; then
        print_status "🎉 All tests passed! 🎉"
        return 0
    else
        print_error "❌ $failed_tests test category(ies) failed ❌"
        return 1
    fi
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "OPTIONS:"
    echo "  --all       Run all tests including coverage, benchmarks, and security audit"
    echo "  --coverage  Run tests with code coverage analysis"
    echo "  --bench     Run performance benchmarks"
    echo "  --security  Run security vulnerability audit"
    echo "  --help      Show this help message"
    echo ""
    echo "EXAMPLES:"
    echo "  $0                    Run standard test suite"
    echo "  $0 --coverage         Run tests with coverage"
    echo "  $0 --all              Run everything"
}

# Parse command line arguments
case "${1:-}" in
    --all)
        main --all
        ;;
    --coverage)
        main --coverage
        ;;
    --bench)
        main --bench
        ;;
    --security)
        main --security
        ;;
    --help|-h)
        show_usage
        exit 0
        ;;
    "")
        main
        ;;
    *)
        echo "Unknown option: $1"
        show_usage
        exit 1
        ;;
esac