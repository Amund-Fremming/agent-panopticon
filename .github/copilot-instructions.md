# Agent Panopticon - AI Agent Instructions

## Architecture Overview

**Agent Panopticon** is a hierarchical multi-agent orchestration system where **Prometheus** (the leader agent) spawns and coordinates 8 specialized agents through a complete development pipeline.

### Core Architecture

- **Single Entry Point**: Always interact through **Prometheus** - he manages all other agents
- **Agent Hierarchy**: Prometheus → 8 specialized agents (Architect, Oracle, Builder, Forge, Atlas, Herald, Refiner, Sentinel)
- **Self-Healing System**: Automatically detects and fixes agent capability issues

### Agent Workflow Pipeline

1. **Architect** - Project planning and task breakdown
2. **Oracle** - Technical specification and stack-wide planning
3. **Builder** - Feature implementation
4. **Forge** - Test development (follows `.github/instructions/test.md.instructions.md`)
5. **Atlas** - Test execution
6. **Herald** - Test result analysis
7. **Refiner** - Code quality improvement
8. **Sentinel** - Final validation and capability issue reporting

## Critical Patterns

### Agent Communication

- **Prometheus Tracking**: All agent activities logged in `.github/context/prometheus.overview.md`
- **Validation Reports**: Sentinel creates reports in `.github/context/validation-reports/` with "Agent Capability Issues" section
- **Self-Healing**: Prometheus reads validation reports and automatically updates agent tool permissions

### Testing Conventions

- **Permission Required**: Before editing `**/*.rs` files, ask "sup my guy, can I [action]?"
- **Test Structure**: Inline `#[cfg(test)] mod tests` with comprehensive edge case coverage
- **Validation**: Sentinel validates all work and reports capability gaps

### File Organization

```
.github/
├── agents/                    # Agent definitions with tool permissions
├── context/
│   ├── prometheus.overview.md # Activity tracking
│   ├── tasks/                 # Task specifications
│   ├── validation-reports/    # Sentinel validation reports
│   └── test-reports/          # Herald test analysis
└── instructions/              # Project-specific rules
```

### Tool Permissions Pattern

Each agent has minimal required tools:

- **Prometheus**: `['edit', 'read', 'search', 'write', 'all']`
- **Atlas**: `['read', 'terminal']` (for test execution)
- **Sentinel**: `['read', 'search', 'terminal']` (for validation)
- Others have task-specific permissions

## Development Workflow

### Task Execution

1. User requests → **Prometheus** analyzes and spawns agents
2. Agents work in sequence through the pipeline
3. **Sentinel** validates and reports issues
4. **Prometheus** fixes capability problems automatically

### Key Integration Points

- **Agent Spawning**: Only Prometheus can spawn other agents
- **Capability Detection**: Sentinel identifies tool permission gaps
- **Self-Improvement**: Prometheus updates agent instructions based on performance

## Code Patterns

### Agent Definition Format

```markdown
---
description: "Agent role description"
tools: ["read", "write", "terminal"]
---

## Role and Responsibilities

[Specific duties]

## Workflow

[Step-by-step process]

## Permissions

[Tool access explanation]
```

### Validation Report Structure

- Requirements compliance table
- Agent performance ratings (⭐⭐⭐⭐⭐)
- "Agent Capability Issues" section (critical for self-healing)
- Recommendations for improvement

## Quality Assurance

- **100% Test Coverage**: Comprehensive edge case testing required
- **Validation Pipeline**: Every task validated by Sentinel
- **Capability Auditing**: Automatic detection of permission issues
- **Performance Tracking**: Agent ratings and improvement recommendations

## Key Files to Reference

- `.github/agents/prometheus-leader.agent.md` - Leader agent definition
- `.github/agents/sentinel-validator.agent.md` - Validation process
- `.github/context/prometheus.overview.md` - Activity tracking
- `.github/instructions/test.md.instructions.md` - Testing rules
