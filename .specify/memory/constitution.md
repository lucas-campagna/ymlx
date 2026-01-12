<!--
Sync Impact Report:
- Version change: 1.0.0 → 1.0.1
- Modified principles: None
- Added sections: None
- Removed sections: None
- Templates requiring updates:
  ✅ plan-template.md (Constitution Check sections aligned with principles)
  ✅ spec-template.md (Parser requirements reflect YMX language specifications)
  ✅ tasks-template.md (Task categorization supports parser-first development)
  ✅ commands (No outdated agent-specific references found)
- Follow-up TODOs: None
-->

# YMX Constitution

## Core Principles

### I. Parser-First Design
YMX is fundamentally a parser system. All features must support and enhance parsing capabilities. Parsing logic must be isolated, testable, and reusable across different component types. Language extensions must maintain backward compatibility with existing parsing rules.

### II. Performance Critical
Parsing performance is non-negotiable. All components must be benchmarked, and parsing operations must complete within strict time limits. Memory usage must be predictable and bounded. No feature may compromise parsing speed or introduce unbounded memory allocation.

### III. Radical Simplicity
YMX language syntax must remain simple and learnable. Component definitions should be intuitive. Implementation must avoid unnecessary abstraction layers. Complex features must be justified with clear user value and simpler alternatives must be considered and documented as rejected.

## Language Specification

### Syntax Stability
Language syntax must follow semantic versioning. Breaking changes require major version increments. All language features must be documented with clear examples and edge case handling.

### Component Contracts
Every component must have a well-defined contract: input format, output format, and error behavior. Components must be independently testable using mock inputs. No component may depend on internal implementation details of other components.

### Interpreter Compatibility
YMX must support multiple interpreters (JavaScript via Boa, Python via RustPython). Language features must work consistently across all supported interpreters. Interpreter-specific behavior must be documented and isolated.

## Performance Standards

### Parsing Benchmarks
All parsing operations must be benchmarked with realistic input sizes. Performance regressions are blocking issues. benchmarks must cover: small components (<1KB), medium components (1-100KB), and large components (>100KB).

### Memory Constraints
Memory usage must scale linearly with input size. No unbounded memory allocation is permitted. Memory usage patterns must be documented for each component type.

### Error Performance
Error detection and reporting must be fast and precise. Parse errors must be reported with accurate line/column information without significantly impacting performance.

## Governance

### Amendment Process
This constitution supersedes all other project practices. Amendments require: (1) proposal documentation, (2) impact analysis on existing components, (3) backward compatibility assessment, and (4) approval through project consensus.

### Versioning Policy
Constitution follows semantic versioning. Major increments for governance changes or principle removal. Minor increments for new sections or materially expanded guidance. Patch increments for clarifications and non-semantic refinements.

### Compliance Review
All code changes must verify compliance with applicable principles. Complexity must be justified in pull requests. Performance implications must be documented. Language extensions must reference this constitution for guidance.

**Version**: 1.0.1 | **Ratified**: 2026-01-12 | **Last Amended**: 2026-01-12