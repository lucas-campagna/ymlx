# YMX API Contracts

**Feature**: YAML Integration System  
**Date**: 2026-01-12  
**Purpose**: API specifications for YMX component processing

## Core Processing API

### Execute Component

Executes a single YMX component with provided context.

**Endpoint**: `POST /api/v1/components/{component_id}/execute`

**Request**:
```json
{
  "component_id": "string",
  "properties": {
    "property_name": "value"
  },
  "execution_context": {
    "interpreter": "javascript|python",
    "memory_limit_mb": 10,
    "timeout_seconds": 5,
    "security_policy": {
      "allow_file_access": false,
      "allow_network_access": false
    }
  }
}
```

**Response**:
```json
{
  "status": "success|error",
  "result": {
    "value": "processed_value",
    "type": "string|number|boolean|array|object"
  },
  "execution_info": {
    "duration_ms": 45,
    "memory_used_mb": 2.1,
    "interpreter_used": "javascript"
  },
  "errors": []
}
```

### Parse YMX File

Parses a YMX file and returns component definitions.

**Endpoint**: `POST /api/v1/parse`

**Request**:
```json
{
  "content": "string (YAML content)",
  "file_path": "string (optional)",
  "options": {
    "strict_mode": true,
    "validate_dependencies": true,
    "include_metadata": true
  }
}
```

**Response**:
```json
{
  "status": "success|error",
  "components": [
    {
      "id": "component_name",
      "name": "Component Name",
      "value": {
        "type": "literal|property_reference|processing_context|component_call|template",
        "content": "string"
      },
      "metadata": {
        "is_template": false,
        "is_generic": false,
        "interpreter": "javascript|python|null",
        "dependencies": ["component1", "component2"]
      },
      "location": {
        "file": "path/to/file.yml",
        "line": 10,
        "column": 5,
        "span": 25
      }
    }
  ],
  "errors": [],
  "warnings": [],
  "metadata": {
    "total_components": 5,
    "parse_time_ms": 12
  }
}
```

### Validate Library

Validates component library for circular dependencies and syntax errors.

**Endpoint**: `POST /api/v1/libraries/validate`

**Request**:
```json
{
  "components": [
    {
      "id": "string",
      "content": "string"
    }
  ],
  "validation_options": {
    "check_circular_dependencies": true,
    "max_nesting_depth": 10,
    "max_component_size_kb": 1024
  }
}
```

**Response**:
```json
{
  "status": "success|error",
  "validation_result": {
    "is_valid": true,
    "errors": [
      {
        "code": "CIRCULAR_DEPENDENCY",
        "message": "Circular dependency detected between A and B",
        "location": {
          "file": "component_a.yml",
          "line": 5,
          "column": 1
        },
        "severity": "error"
      }
    ],
    "warnings": [],
    "dependency_graph": {
      "nodes": ["component_a", "component_b"],
      "edges": [
        {
          "from": "component_a",
          "to": "component_b",
          "type": "direct_call"
        }
      ]
    }
  }
}
```

## Library Management API

### List Components

Retrieves list of available components in a library.

**Endpoint**: `GET /api/v1/libraries/{library_id}/components`

**Query Parameters**:
- `format`: `json|yaml|table` (default: json)
- `filter`: `templates|generics|regular` (optional)
- `sort`: `name|size|dependencies` (default: name)

**Response**:
```json
{
  "library_id": "string",
  "components": [
    {
      "id": "component_name",
      "name": "Component Name",
      "type": "template|generic|regular",
      "dependencies_count": 2,
      "size_bytes": 1024,
      "last_modified": "2026-01-12T10:00:00Z"
    }
  ],
  "metadata": {
    "total_count": 15,
    "filtered_count": 10
  }
}
```

### Get Component Details

Retrieves detailed information about a specific component.

**Endpoint**: `GET /api/v1/libraries/{library_id}/components/{component_id}`

**Response**:
```json
{
  "id": "component_name",
  "content": "string (YAML content)",
  "metadata": {
    "is_template": false,
    "is_generic": false,
    "interpreter": "javascript|null",
    "dependencies": ["component1"],
    "dependents": ["component2", "component3"],
    "created_at": "2026-01-12T10:00:00Z",
    "updated_at": "2026-01-12T15:30:00Z"
  },
  "execution_info": {
    "average_duration_ms": 25,
    "success_rate": 0.98,
    "last_execution": "2026-01-12T14:20:00Z"
  }
}
```

### Create/Update Component

Adds or updates a component in the library.

**Endpoint**: `PUT /api/v1/libraries/{library_id}/components/{component_id}`

**Request**:
```json
{
  "content": "string (YAML content)",
  "metadata": {
    "description": "string (optional)",
    "tags": ["tag1", "tag2"]
  }
}
```

**Response**:
```json
{
  "status": "created|updated",
  "component_id": "string",
  "version": "string",
  "created_at": "2026-01-12T16:00:00Z"
}
```

## Batch Operations API

### Execute Multiple Components

Executes multiple components in parallel or sequence.

**Endpoint**: `POST /api/v1/batch/execute`

**Request**:
```json
{
  "executions": [
    {
      "component_id": "component1",
      "properties": {},
      "execution_context": {}
    },
    {
      "component_id": "component2",
      "properties": {},
      "execution_context": {}
    }
  ],
  "execution_mode": "parallel|sequence",
  "global_context": {
    "timeout_seconds": 30,
    "memory_limit_mb": 50
  }
}
```

**Response**:
```json
{
  "status": "success|partial_success|error",
  "results": [
    {
      "component_id": "component1",
      "status": "success|error",
      "result": {},
      "execution_info": {
        "duration_ms": 45,
        "start_time": "2026-01-12T16:00:00Z",
        "end_time": "2026-01-12T16:00:00.045Z"
      },
      "errors": []
    }
  ],
  "summary": {
    "total_executions": 2,
    "successful": 1,
    "failed": 1,
    "total_duration_ms": 120
  }
}
```

## Configuration API

### Get Configuration

Retrieves current system configuration.

**Endpoint**: `GET /api/v1/config`

**Response**:
```json
{
  "cli_config": {
    "default_output_format": "json",
    "timeout_seconds": 30,
    "memory_limit_mb": 100,
    "component_paths": ["/path/to/components"],
    "security_policy": {
      "allow_file_access": false,
      "allow_network_access": false,
      "max_execution_time_ms": 5000,
      "max_memory_mb": 10
    }
  },
  "wasm_config": {
    "enable_features": {
      "simd": true,
      "threads": false,
      "bulk_memory": true
    },
    "performance_mode": "balanced",
    "security_level": "standard"
  }
}
```

### Update Configuration

Updates system configuration.

**Endpoint**: `PUT /api/v1/config`

**Request**:
```json
{
  "cli_config": {
    "default_output_format": "yaml",
    "timeout_seconds": 60
  }
}
```

**Response**:
```json
{
  "status": "success",
  "updated_fields": ["cli_config.default_output_format", "cli_config.timeout_seconds"],
  "validation_warnings": []
}
```

## Health and Monitoring API

### Health Check

Returns system health status and capabilities.

**Endpoint**: `GET /api/v1/health`

**Response**:
```json
{
  "status": "healthy|degraded|unhealthy",
  "version": "1.0.0",
  "uptime_seconds": 3600,
  "capabilities": {
    "javascript_interpreter": true,
    "python_interpreter": true,
    "wasm_compilation": true,
    "parallel_execution": true
  },
  "performance_metrics": {
    "average_parse_time_ms": 15,
    "average_execution_time_ms": 35,
    "memory_usage_mb": 45,
    "active_components": 127
  },
  "limits": {
    "max_component_size_mb": 10,
    "max_concurrent_executions": 100,
    "max_execution_time_seconds": 300
  }
}
```

### Performance Metrics

Returns detailed performance metrics and statistics.

**Endpoint**: `GET /api/v1/metrics`

**Query Parameters**:
- `period`: `1h|24h|7d|30d` (default: 24h)
- `granularity`: `minute|hour|day` (default: hour)

**Response**:
```json
{
  "period": "24h",
  "granularity": "hour",
  "metrics": {
    "executions": {
      "total": 15420,
      "successful": 15120,
      "failed": 300,
      "success_rate": 0.98,
      "average_duration_ms": 32.5
    },
    "performance": {
      "p50_duration_ms": 25,
      "p95_duration_ms": 75,
      "p99_duration_ms": 120,
      "max_duration_ms": 250
    },
    "resources": {
      "peak_memory_mb": 78,
      "average_memory_mb": 45,
      "cpu_usage_percent": 15.2
    }
  },
  "trends": [
    {
      "timestamp": "2026-01-12T00:00:00Z",
      "executions": 642,
      "average_duration_ms": 30.1,
      "success_rate": 0.99
    }
  ]
}
```

## Error Response Format

All error responses follow this consistent format:

```json
{
  "status": "error",
  "error": {
    "code": "string (error code)",
    "message": "string (human readable)",
    "details": {
      "field": "string (if applicable)",
      "value": "string (if applicable)",
      "constraints": "string (if applicable)"
    }
  },
  "request_id": "string (for tracing)",
  "timestamp": "2026-01-12T16:00:00Z"
}
```

## Error Codes

### Parse Errors
- `PARSE_SYNTAX_ERROR`: Invalid YAML syntax
- `PARSE_PROPERTY_ERROR`: Invalid property reference
- `PARSE_CONTEXT_ERROR`: Invalid processing context

### Execution Errors
- `EXECUTION_TIMEOUT`: Component exceeded time limit
- `EXECUTION_MEMORY_LIMIT`: Component exceeded memory limit
- `EXECUTION_SECURITY_VIOLATION`: Security policy violation
- `EXECUTION_INTERPRETER_ERROR`: Interpreter runtime error

### Library Errors
- `LIBRARY_COMPONENT_NOT_FOUND`: Component does not exist
- `LIBRARY_CIRCULAR_DEPENDENCY`: Circular dependency detected
- `LIBRARY_VALIDATION_ERROR`: Component validation failed

### System Errors
- `SYSTEM_CONFIGURATION_ERROR`: Invalid configuration
- `SYSTEM_RESOURCE_EXHAUSTED`: System resources exhausted
- `SYSTEM_INTERNAL_ERROR`: Internal system error

These API contracts provide a comprehensive interface for YMX component processing while maintaining consistency across CLI and WASM deployments.