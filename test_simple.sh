# Test with simplified components
cat > test_simple.yml << 'EOF'
component: Hello $name!
result: $default!
EOF

# Test with available functions
if ./target/debug/ymx parse test_simple.yml --component component --name "World" > /dev/null 2>&1; then
    echo "SUCCESS: Basic functionality works"
else
    echo "FAILED: Basic functionality"
fi