use std::time::{Duration, Instant};
use std::collections::HashMap;
use ymx::*;
use ymx::component::*;
use crate::fixtures::*;

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_parsing_performance_small_yaml() {
        let content = generate_test_yaml(100);
        let iterations = 1000;
        
        let (duration, total_components) = benchmark_parsing(iterations, &content);
        
        println!("Parsed {} components {} times in {:?}", 
                total_components / iterations, iterations, duration);
        
        // Should parse quickly (less than 1 second for 100 components * 1000 iterations)
        assert!(duration.as_secs() < 1, 
               "Small YAML parsing too slow: {:?}", duration);
    }

    #[test]
    fn test_parsing_performance_medium_yaml() {
        let content = generate_test_yaml(1000);
        let iterations = 100;
        
        let (duration, total_components) = benchmark_parsing(iterations, &content);
        
        println!("Parsed {} components {} times in {:?}", 
                total_components / iterations, iterations, duration);
        
        // Should handle medium YAML efficiently (less than 5 seconds for 1000 components * 100 iterations)
        assert!(duration.as_secs() < 5, 
               "Medium YAML parsing too slow: {:?}", duration);
    }

    #[test]
    fn test_parsing_performance_large_yaml() {
        let content = generate_test_yaml(5000);
        let iterations = 10;
        
        let (duration, total_components) = benchmark_parsing(iterations, &content);
        
        println!("Parsed {} components {} times in {:?}", 
                total_components / iterations, iterations, duration);
        
        // Should handle large YAML (less than 10 seconds for 5000 components * 10 iterations)
        assert!(duration.as_secs() < 10, 
               "Large YAML parsing too slow: {:?}", duration);
    }

    #[test]
    fn test_execution_performance_simple_literals() {
        let component = create_test_component(
            "test",
            ComponentValue::Literal("Simple literal value".to_string())
        );
        let context = create_test_context();
        let iterations = 10000;
        
        let duration = benchmark_execution(iterations, &component, &context);
        let per_execution = duration.as_nanos() as f64 / iterations as f64;
        
        println!("Executed {} literal components in {:?} ({:.2} ns per execution)", 
                iterations, duration, per_execution);
        
        // Literal execution should be very fast (< 1000 ns per execution)
        assert!(per_execution < 1000.0, 
               "Literal execution too slow: {:.2} ns", per_execution);
    }

    #[test]
    fn test_execution_performance_property_references() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("name".to_string())
        );
        let context = create_test_context();
        let iterations = 10000;
        
        let duration = benchmark_execution(iterations, &component, &context);
        let per_execution = duration.as_nanos() as f64 / iterations as f64;
        
        println!("Executed {} property reference components in {:?} ({:.2} ns per execution)", 
                iterations, duration, per_execution);
        
        // Property reference execution should be fast (< 5000 ns per execution)
        assert!(per_execution < 5000.0, 
               "Property reference execution too slow: {:.2} ns", per_execution);
    }

    #[test]
    fn test_execution_performance_processing_contexts() {
        let component = create_test_component(
            "test",
            ComponentValue::ProcessingContext("1 + 2 + 3".to_string())
        );
        let context = create_test_context();
        let iterations = 1000;
        
        let duration = benchmark_execution(iterations, &component, &context);
        let per_execution = duration.as_nanos() as f64 / iterations as f64;
        
        println!("Executed {} processing context components in {:?} ({:.2} ns per execution)", 
                iterations, duration, per_execution);
        
        // Processing context execution can be slower (< 50000 ns per execution)
        assert!(per_execution < 50000.0, 
               "Processing context execution too slow: {:.2} ns", per_execution);
    }

    #[test]
    fn test_execution_performance_component_calls() {
        let mut properties = HashMap::new();
        properties.insert("text".to_string(), ComponentValue::Literal("Click Me".to_string()));
        properties.insert("style".to_string(), ComponentValue::Literal("primary".to_string()));
        
        let component = create_test_component(
            "test",
            ComponentValue::ComponentCall(ComponentCall {
                target: "base_button".to_string(),
                properties,
            })
        );
        let context = create_test_context();
        let iterations = 1000;
        
        let duration = benchmark_execution(iterations, &component, &context);
        let per_execution = duration.as_nanos() as f64 / iterations as f64;
        
        println!("Executed {} component call in {:?} ({:.2} ns per execution)", 
                iterations, duration, per_execution);
        
        // Component calls can be slower (< 100000 ns per execution)
        assert!(per_execution < 100000.0, 
               "Component call execution too slow: {:.2} ns", per_execution);
    }

    #[test]
    fn test_substitution_performance_simple_strings() {
        let template = "Hello World!";
        let context = create_test_context();
        let iterations = 10000;
        
        let start = Instant::now();
        for _ in 0..iterations {
            let result = substitute_in_string(template, &context);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Hello World!");
        }
        let duration = start.elapsed();
        let per_substitution = duration.as_nanos() as f64 / iterations as f64;
        
        println!("Substituted {} simple strings in {:?} ({:.2} ns per substitution)", 
                iterations, duration, per_substitution);
        
        // Simple string substitution should be very fast (< 1000 ns)
        assert!(per_substitution < 1000.0, 
               "Simple substitution too slow: {:.2} ns", per_substitution);
    }

    #[test]
    fn test_substitution_performance_with_properties() {
        let template = "Hello $name! You are $age years old from $city.";
        let context = create_test_context();
        let iterations = 10000;
        
        let start = Instant::now();
        for _ in 0..iterations {
            let result = substitute_in_string(template, &context);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Hello World! You are 25 years old from New York.");
        }
        let duration = start.elapsed();
        let per_substitution = duration.as_nanos() as f64 / iterations as f64;
        
        println!("Substituted {} strings with properties in {:?} ({:.2} ns per substitution)", 
                iterations, duration, per_substitution);
        
        // Property substitution should be fast (< 5000 ns)
        assert!(per_substitution < 5000.0, 
               "Property substitution too slow: {:.2} ns", per_substitution);
    }

    #[test]
    fn test_substitution_performance_processing_contexts() {
        let template = "Result: ${1 + 2} and ${user.age + 10}";
        let context = create_test_context();
        let iterations = 1000;
        
        let start = Instant::now();
        for _ in 0..iterations {
            let result = substitute_in_string(template, &context);
            assert!(result.is_ok());
            assert!(result.unwrap().contains("1 + 2"));
            assert!(result.unwrap().contains("user.age + 10"));
        }
        let duration = start.elapsed();
        let per_substitution = duration.as_nanos() as f64 / iterations as f64;
        
        println!("Substituted {} strings with processing contexts in {:?} ({:.2} ns per substitution)", 
                iterations, duration, per_substitution);
        
        // Processing context substitution can be slower (< 20000 ns)
        assert!(per_substitution < 20000.0, 
               "Processing context substitution too slow: {:.2} ns", per_substitution);
    }

    #[test]
    fn test_memory_usage_parsing() {
        let initial_memory = estimate_memory_usage(&());
        
        // Parse and immediately drop many components
        for i in 0..100 {
            let content = generate_test_yaml(100);
            let result = parse_yaml_content(&content);
            assert!(result.is_ok());
            let components = result.unwrap();
            assert_eq!(components.len(), 100);
            
            // Components should be dropped here
            std::mem::drop(components);
        }
        
        // Memory usage should not grow significantly
        // Note: This is a simplified test - real memory profiling would need better tools
        let final_memory = estimate_memory_usage(&());
        println!("Memory usage - Initial: {}, Final: {}", initial_memory, final_memory);
    }

    #[test]
    fn test_memory_usage_execution() {
        let component = create_test_component(
            "test",
            ComponentValue::Literal("Test value".to_string())
        );
        let context = create_test_context();
        let initial_memory = estimate_memory_usage(&component);
        
        // Execute many times
        for _ in 0..10000 {
            let result = component::execute_component(&component, &context);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), "Test value");
        }
        
        let final_memory = estimate_memory_usage(&component);
        println!("Memory usage - Initial: {}, Final: {}", initial_memory, final_memory);
    }

    #[test]
    fn test_scalability_linear_parsing() {
        let sizes = vec![100, 500, 1000, 2000, 5000];
        let mut previous_time = Duration::ZERO;
        
        for &size in &sizes {
            let content = generate_test_yaml(size);
            let iterations = 10.max(1000 / size); // Adjust iterations based on size
            
            let (duration, _) = benchmark_parsing(iterations, &content);
            let per_component = duration.as_nanos() as f64 / (size * iterations) as f64;
            
            println!("Size: {}, Per component: {:.2} ns", size, per_component);
            
            // Performance should scale roughly linearly
            if previous_time != Duration::ZERO {
                let ratio = duration.as_nanos() as f64 / previous_time.as_nanos() as f64;
                let size_ratio = size as f64 / (sizes[sizes.iter().position(|&x| x < size).unwrap_or(0)] as f64);
                
                // Allow some non-linearity due to fixed overhead
                assert!(ratio < size_ratio * 2.0, 
                       "Performance doesn't scale linearly: ratio = {:.2}, size_ratio = {:.2}", 
                       ratio, size_ratio);
            }
            
            previous_time = duration;
        }
    }

    #[test]
    fn test_scalability_linear_execution() {
        let context_sizes = vec![10, 100, 1000, 5000];
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("test_prop".to_string())
        );
        
        for &size in &context_sizes {
            let mut context = HashMap::new();
            for i in 0..size {
                context.insert(format!("key_{}", i), format!("value_{}", i));
            }
            context.insert("test_prop".to_string(), format!("value_for_size_{}", size));
            
            let iterations = 1000;
            let start = Instant::now();
            
            for _ in 0..iterations {
                let result = component::execute_component(&component, &context);
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), format!("value_for_size_{}", size));
            }
            
            let duration = start.elapsed();
            let per_execution = duration.as_nanos() as f64 / iterations as f64;
            
            println!("Context size: {}, Per execution: {:.2} ns", size, per_execution);
            
            // Performance should not degrade too much with context size
            assert!(per_execution < 100000.0, 
                   "Execution too slow for context size {}: {:.2} ns", size, per_execution);
        }
    }

    #[test]
    fn test_concurrent_parsing_performance() {
        use std::sync::Arc;
        use std::thread;
        
        let content = Arc::new(generate_test_yaml(1000));
        let thread_count = 4;
        let iterations_per_thread = 25;
        
        let start = Instant::now();
        let mut handles = vec![];
        
        for _ in 0..thread_count {
            let content_clone = Arc::clone(&content);
            let handle = thread::spawn(move || {
                for _ in 0..iterations_per_thread {
                    let result = parse_yaml_content(&content_clone);
                    assert!(result.is_ok());
                    let components = result.unwrap();
                    assert_eq!(components.len(), 1000);
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let total_duration = start.elapsed();
        let total_iterations = thread_count * iterations_per_thread;
        let per_parse = total_duration.as_nanos() as f64 / total_iterations as f64;
        
        println!("Concurrent parsing: {} threads, {} iterations, total time: {:?} ({:.2} ns per parse)", 
                thread_count, total_iterations, total_duration, per_parse);
        
        // Concurrent parsing should be efficient
        assert!(per_parse < 1000000.0, // 1ms per parse
               "Concurrent parsing too slow: {:.2} ns", per_parse);
    }

    #[test]
    fn test_concurrent_execution_performance() {
        use std::sync::Arc;
        use std::thread;
        
        let component = Arc::new(create_test_component(
            "test",
            ComponentValue::PropertyReference("name".to_string())
        ));
        let context = Arc::new(create_test_context());
        let thread_count = 8;
        let iterations_per_thread = 1000;
        
        let start = Instant::now();
        let mut handles = vec![];
        
        for _ in 0..thread_count {
            let component_clone = Arc::clone(&component);
            let context_clone = Arc::clone(&context);
            
            let handle = thread::spawn(move || {
                for _ in 0..iterations_per_thread {
                    let result = component::execute_component(&component_clone, &context_clone);
                    assert!(result.is_ok());
                    assert_eq!(result.unwrap(), "World");
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let total_duration = start.elapsed();
        let total_iterations = thread_count * iterations_per_thread;
        let per_execution = total_duration.as_nanos() as f64 / total_iterations as f64;
        
        println!("Concurrent execution: {} threads, {} iterations, total time: {:?} ({:.2} ns per execution)", 
                thread_count, total_iterations, total_duration, per_execution);
        
        // Concurrent execution should be fast
        assert!(per_execution < 10000.0, // 10 microseconds per execution
               "Concurrent execution too slow: {:.2} ns", per_execution);
    }

    #[test]
    fn test_large_string_processing() {
        let large_string = "x".repeat(10000);
        let component = create_test_component(
            "test",
            ComponentValue::Literal(large_string.clone())
        );
        let context = create_test_context();
        let iterations = 100;
        
        let start = Instant::now();
        for _ in 0..iterations {
            let result = component::execute_component(&component, &context);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), large_string);
        }
        let duration = start.elapsed();
        let per_execution = duration.as_nanos() as f64 / iterations as f64;
        
        println!("Large string processing: {} chars, {:.2} ns per execution", 
                large_string.len(), per_execution);
        
        // Large string processing should be reasonable
        assert!(per_execution < 10000000.0, // 10ms per execution
               "Large string processing too slow: {:.2} ns", per_execution);
    }

    #[test]
    fn test_unicode_processing_performance() {
        let unicode_string = "🚀 你好世界 🌍 مرحبا بالعالم 🎨🖼️🎭🎪".repeat(100);
        let component = create_test_component(
            "test",
            ComponentValue::Literal(unicode_string.clone())
        );
        let context = create_test_context();
        let iterations = 1000;
        
        let start = Instant::now();
        for _ in 0..iterations {
            let result = component::execute_component(&component, &context);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), unicode_string);
        }
        let duration = start.elapsed();
        let per_execution = duration.as_nanos() as f64 / iterations as f64;
        
        println!("Unicode processing: {} chars, {:.2} ns per execution", 
                unicode_string.chars().count(), per_execution);
        
        // Unicode processing should be reasonably fast
        assert!(per_execution < 500000.0, // 0.5ms per execution
               "Unicode processing too slow: {:.2} ns", per_execution);
    }

    #[test]
    fn test_complex_substitution_performance() {
        let template = format!("Hello $name1! Age: $age1. City: $city1. Status: $status1. Count: $count1. " +
                       "Hello $name2! Age: $age2. City: $city2. Status: $status2. Count: $count2. " +
                       "Math: ${age1 + age2}. String: ${\"Hello, \" + name1 + \" and \" + name2 + \"!\"}. " +
                       "Ternary: ${status1 ? \"Active\" : \"Inactive\"}. Array: $items[0].", 
                       i, i, i, i, i, i, i, i, i, i, i, i, i, i);
        
        let mut context = HashMap::new();
        context.insert("name1".to_string(), "Alice".to_string());
        context.insert("name2".to_string(), "Bob".to_string());
        context.insert("age1".to_string(), "25".to_string());
        context.insert("age2".to_string(), "30".to_string());
        context.insert("city1".to_string(), "New York".to_string());
        context.insert("city2".to_string(), "Paris".to_string());
        context.insert("status1".to_string(), "true".to_string());
        context.insert("status2".to_string(), "false".to_string());
        context.insert("count1".to_string(), "42".to_string());
        context.insert("count2".to_string(), "100".to_string());
        context.insert("items".to_string(), "item1,item2,item3".to_string());
        
        let iterations = 1000;
        let start = Instant::now();
        
        for _ in 0..iterations {
            let result = substitute_in_string(template, &context);
            assert!(result.is_ok());
        }
        
        let duration = start.elapsed();
        let per_substitution = duration.as_nanos() as f64 / iterations as f64;
        
        println!("Complex substitution: {:.2} ns per substitution", per_substitution);
        
        // Complex substitution should still be reasonably fast
        assert!(per_substitution < 100000.0, // 100 microseconds
               "Complex substitution too slow: {:.2} ns", per_substitution);
    }

    #[test]
    fn test_stress_test_multiple_operations() {
        let iterations = 100;
        
        for i in 0..iterations {
            // Generate different content for each iteration
            let content = format!(r#"
component_{}: value_{}
prop_ref_{}: $prop_{}
expr_{}: ${{1 + {}}}
call_{}:
  from!: base_{}
  text: Click {}
"#, i, i, i, i, i, i, i, i, i, i, i, i);
            
            let result = parse_yaml_content(&content);
            assert!(result.is_ok());
            let components = result.unwrap();
            assert_eq!(components.len(), 5);
            
            let mut context = HashMap::new();
            context.insert(format!("prop_{}", i), format!("value_{}", i));
            
            // Execute each component
            for component in &components {
                let exec_result = component::execute_component(component, &context);
                assert!(exec_result.is_ok());
            }
            
            // Test substitution
            let template = format!("Iteration {}: $prop_{} and ${{2 + {}}}", i, i, i);
            let sub_result = substitute_in_string(template, &context);
            assert!(sub_result.is_ok());
        }
        
        println!("Stress test completed: {} iterations with multiple operations", iterations);
    }

    #[test]
    fn test_memory_leak_detection() {
        // Run many operations and check for memory leaks
        let initial_memory = estimate_memory_usage(&());
        
        for _ in 0..100 {
            // Parse large YAML
            let content = generate_test_yaml(1000);
            let result = parse_yaml_content(&content);
            assert!(result.is_ok());
            let components = result.unwrap();
            
            // Execute all components
            let context = create_test_context();
            for component in &components {
                let exec_result = component::execute_component(component, &context);
                assert!(exec_result.is_ok());
            }
            
            // All should be dropped here
        }
        
        let final_memory = estimate_memory_usage(&());
        
        // In a real scenario, we'd use proper memory profiling tools
        // This is a basic check
        println!("Memory leak test - Initial: {}, Final: {}", initial_memory, final_memory);
        
        // Memory usage should not grow dramatically
        // Note: This is simplified - real leak detection needs better tools
        assert!(final_memory <= initial_memory * 2, 
               "Potential memory leak detected: {} -> {}", initial_memory, final_memory);
    }

    #[test]
    fn test_benchmark_report() {
        println!("\n=== YMX Performance Benchmark Report ===\n");
        
        // Parsing benchmarks
        println!("Parsing Performance:");
        for &size in &[100, 1000, 5000] {
            let content = generate_test_yaml(size);
            let iterations = 100.max(1000 / size);
            let (duration, _) = benchmark_parsing(iterations, &content);
            let per_component = duration.as_micros() as f64 / (size * iterations) as f64;
            println!("  {} components: {:.2} μs per component", size, per_component);
        }
        
        // Execution benchmarks
        println!("\nExecution Performance:");
        
        let literal_component = create_test_component("test", ComponentValue::Literal("test".to_string()));
        let prop_component = create_test_component("test", ComponentValue::PropertyReference("test".to_string()));
        let expr_component = create_test_component("test", ComponentValue::ProcessingContext("1 + 2".to_string()));
        let context = create_test_context();
        
        for (name, component) in [
            ("Literal", &literal_component),
            ("Property Reference", &prop_component),
            ("Processing Context", &expr_component),
        ] {
            let iterations = 10000;
            let duration = benchmark_execution(iterations, component, &context);
            let per_execution = duration.as_nanos() as f64 / iterations as f64;
            println!("  {}: {:.2} ns per execution", name, per_execution);
        }
        
        // Substitution benchmarks
        println!("\nSubstitution Performance:");
        let templates = [
            ("No substitution", "Hello World!"),
            ("Single property", "Hello $name!"),
            ("Multiple properties", "Hello $name! Age: $age. City: $city."),
            ("With processing context", "Result: ${1 + 2} and ${user.age + 10}"),
        ];
        
        let context = create_test_context();
        let iterations = 10000;
        
        for (name, template) in &templates {
            let start = Instant::now();
            for _ in 0..iterations {
                let result = substitute_in_string(template, &context);
                assert!(result.is_ok());
            }
            let duration = start.elapsed();
            let per_substitution = duration.as_nanos() as f64 / iterations as f64;
            println!("  {}: {:.2} ns per substitution", name, per_substitution);
        }
        
        println!("\n=== End Benchmark Report ===\n");
    }
}