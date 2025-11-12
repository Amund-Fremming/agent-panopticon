# Agent Panopticon

![Agent Panopticon](agent-pano.png)

An experimental project exploring autonomous agent orchestration through a hierarchical system led by **Prometheus**, the leader agent.

## Overview

Agent Panopticon is a self-improving multi-agent development system where specialized AI agents collaborate to complete software development tasks from planning through validation.

## Architecture

### Leader Agent
- **Prometheus** - The orchestrator with full system access who spawns, manages, and coordinates all other agents

### Specialized Agents
- **Architect** - Project management and task breakdown
- **Oracle** - Technical leadership and stack-wide implementation planning
- **Builder** - Feature development and implementation
- **Forge** - Test development and creation
- **Atlas** - Test execution and monitoring
- **Herald** - Test reporting and analysis
- **Refiner** - Code quality improvement and refactoring
- **Sentinel** - Solution validation and quality assurance

## How It Works

1. User interacts only with **Prometheus**
2. Prometheus analyzes the request and spawns appropriate specialized agents
3. Agents work in coordinated sequence: Plan → Build → Test → Validate
4. Sentinel validates all work and creates detailed reports
5. Prometheus reads validation reports and automatically fixes agent capability issues
6. The system self-improves by updating agent instructions based on performance

## Key Features

- **Single Interface**: Interact only with Prometheus, who manages all other agents
- **Self-Healing**: Automatically detects and fixes agent permission issues
- **Complete Workflow**: From project planning to validated, tested code
- **Transparency**: All agent activities tracked in `.github/context/prometheus.overview.md`
- **Quality Assurance**: Every task validated with detailed performance reports

## Agent Files

All agent definitions are located in `.github/agents/` with descriptive names:
- `prometheus-leader.agent.md`
- `architect-project-manager.agent.md`
- `oracle-tech-lead.agent.md`
- And more...

## Validation Reports

After each task, Sentinel creates comprehensive validation reports in `.github/context/validation-reports/` that assess:
- Requirement compliance
- Code quality
- Test coverage
- Individual agent performance
- Recommendations for improvements

## Status

🚧 Experimental - Exploring autonomous agent orchestration patterns