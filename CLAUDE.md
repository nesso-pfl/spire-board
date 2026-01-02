# CLAUDE.md - AI Assistant Guide for spire-board

This document provides comprehensive guidance for AI assistants working on the spire-board codebase.

## Repository Overview

**Project Name:** spire-board
**Repository:** nesso-pfl/spire-board
**Current Status:** Initial setup phase

### Project Description

This is a new project currently in the initial development phase. As the codebase evolves, this section should be updated with:
- Project purpose and goals
- Target users/audience
- Key features and functionality
- Technology stack

## Codebase Structure

### Directory Organization

As the project develops, document the directory structure here:

```
spire-board/
├── src/              # Source code
├── tests/            # Test files
├── docs/             # Documentation
├── config/           # Configuration files
└── scripts/          # Build and utility scripts
```

**Note:** Update this structure as directories are created.

### Key Files and Their Purposes

Document important files as they are created:
- Configuration files (package.json, tsconfig.json, etc.)
- Entry points
- Core modules
- Build scripts

## Development Workflows

### Branch Strategy

**Main Development Branch:** TBD (typically `main` or `master`)
**Feature Branches:** Use pattern `claude/feature-name-{sessionId}`

#### Branch Guidelines

1. **Never push directly to the main branch** without explicit permission
2. **Always work on feature branches** named with the `claude/` prefix
3. **Branch naming convention:** `claude/descriptive-name-{sessionId}`
4. **Keep branches focused:** One feature or fix per branch

### Git Workflow

#### Making Changes

1. **Before starting work:**
   ```bash
   git fetch origin
   git checkout -b claude/feature-name-{sessionId}
   ```

2. **During development:**
   - Make focused, logical commits
   - Write clear commit messages (see conventions below)
   - Commit related changes together

3. **Before pushing:**
   ```bash
   git status  # Review changes
   git diff    # Inspect modifications
   ```

4. **Pushing changes:**
   ```bash
   git push -u origin claude/feature-name-{sessionId}
   ```
   - **Critical:** Branch must start with `claude/` and end with session ID
   - Retry on network failures: up to 4 times with exponential backoff (2s, 4s, 8s, 16s)

#### Commit Message Conventions

Follow these patterns for commit messages:

- **Features:** `feat: add user authentication system`
- **Bug fixes:** `fix: resolve memory leak in data processing`
- **Refactoring:** `refactor: simplify error handling logic`
- **Documentation:** `docs: update API documentation`
- **Tests:** `test: add unit tests for validation module`
- **Chores:** `chore: update dependencies`

**Format:**
```
<type>: <short description>

<optional detailed description>

<optional breaking changes note>
```

### Pull Request Process

1. **Create descriptive PRs** with:
   - Clear title summarizing the change
   - Detailed description of what changed and why
   - Test plan or verification steps
   - Reference to related issues

2. **PR Description Template:**
   ```markdown
   ## Summary
   - Brief overview of changes
   - Why these changes were needed

   ## Changes Made
   - List specific modifications
   - Files affected

   ## Test Plan
   - [ ] Steps to verify the changes
   - [ ] Edge cases considered
   - [ ] Existing tests still pass
   ```

## Coding Conventions

### General Principles

1. **Simplicity First**
   - Avoid over-engineering
   - Don't add features beyond what's requested
   - Keep solutions focused and minimal

2. **Code Quality**
   - Write self-documenting code with clear variable/function names
   - Add comments only where logic isn't self-evident
   - Follow existing patterns in the codebase

3. **Security**
   - Prevent common vulnerabilities (XSS, SQL injection, command injection)
   - Validate input at system boundaries
   - Use parameterized queries for databases
   - Sanitize user input appropriately

### Language-Specific Conventions

**As the project develops, document conventions for:**
- Naming conventions (files, functions, variables, classes)
- Code organization patterns
- Import/export styles
- Error handling approaches
- Testing patterns

### Testing Requirements

- Write tests for new functionality
- Maintain or improve code coverage
- Run full test suite before committing
- Include both unit and integration tests where appropriate

## AI Assistant Guidelines

### Before Making Changes

1. **Always read files before modifying them**
   - Use Read tool to understand existing code
   - Never propose changes to unread code
   - Understand context and patterns

2. **Research first**
   - Use Explore agent for codebase exploration
   - Search for existing patterns and similar implementations
   - Check for related functionality

### Task Management

1. **Use TodoWrite tool** for multi-step tasks:
   - Break complex work into steps
   - Track progress in real-time
   - Mark tasks complete immediately after finishing

2. **One task in progress at a time**
   - Update status as you work
   - Complete current task before starting new ones

### Making Changes

1. **Prefer editing over creating**
   - Always edit existing files when possible
   - Only create new files when absolutely necessary
   - Don't create documentation unless requested

2. **Minimize changes**
   - Only modify what's needed for the task
   - Don't refactor surrounding code unless asked
   - Don't add "improvements" beyond the request
   - Avoid adding extra error handling for impossible scenarios

3. **Follow existing patterns**
   - Match the style of surrounding code
   - Use established patterns in the codebase
   - Don't introduce new patterns without good reason

### Code Review Checklist

Before committing, verify:
- [ ] All requested functionality is implemented
- [ ] No unrelated changes included
- [ ] Security vulnerabilities addressed
- [ ] Tests pass
- [ ] Code follows project conventions
- [ ] No backwards-compatibility hacks
- [ ] Unused code removed completely (not commented out)

### Communication

1. **Be concise and clear**
   - Provide direct technical information
   - Focus on facts and problem-solving
   - Avoid unnecessary praise or superlatives

2. **Show your work**
   - Explain what you're doing and why
   - Reference specific files and line numbers: `file_path:line_number`
   - Provide context for decisions

## Common Tasks

### Project Setup

**To be documented as project infrastructure is established:**
- Installation steps
- Environment configuration
- Dependencies management
- Build process

### Running the Project

**To be documented:**
- Development server commands
- Build commands
- Test commands
- Linting and formatting

### Debugging

**Best practices:**
- Use appropriate debugging tools for the stack
- Check logs and error messages
- Reproduce issues before fixing
- Add tests to prevent regression

## Technology Stack

**To be documented as technologies are chosen:**
- Primary language(s)
- Frameworks and libraries
- Build tools
- Testing frameworks
- Database systems
- Deployment platforms

## Architecture Patterns

**Document as the architecture emerges:**
- Design patterns in use
- State management approach
- API design principles
- Data flow patterns
- Module organization

## Dependencies

**Track important dependencies:**
- Core runtime dependencies
- Development dependencies
- Version constraints and compatibility notes
- Known issues or workarounds

## Environment Setup

**Document environment requirements:**
- Required tools and versions
- Environment variables
- Configuration files
- IDE/editor recommendations

## Troubleshooting

### Common Issues

**Document common problems and solutions as they arise:**

| Issue | Solution |
|-------|----------|
| TBD   | TBD      |

### Getting Help

- Check existing documentation
- Search codebase for similar implementations
- Review git history for context
- Ask specific questions with context

## Performance Considerations

**Document as performance requirements emerge:**
- Critical performance paths
- Optimization strategies
- Profiling approaches
- Known bottlenecks

## Security Guidelines

1. **Input Validation**
   - Validate at system boundaries
   - Sanitize user input
   - Use allowlists over denylists

2. **Authentication & Authorization**
   - Follow established auth patterns
   - Never hardcode credentials
   - Use environment variables for secrets

3. **Data Protection**
   - Encrypt sensitive data
   - Use secure communication protocols
   - Follow principle of least privilege

## Maintenance

### Updating This Document

This CLAUDE.md should be updated when:
- Project structure changes significantly
- New conventions are established
- Technologies are added or changed
- Common issues are discovered
- Workflows are modified

### Review Schedule

- Review quarterly or after major changes
- Keep information current and accurate
- Remove outdated information
- Add newly discovered patterns and practices

## Quick Reference

### Essential Commands

```bash
# To be filled in as project develops
git status              # Check repository status
git diff                # Review changes
git log --oneline -10   # Recent commits
```

### File Locations

**Update as project structure is established:**
- Tests: TBD
- Configuration: TBD
- Documentation: TBD
- Build output: TBD

### Useful Patterns

**Document reusable code patterns as they emerge:**
- Error handling
- API calls
- State updates
- Component structure

---

## Document Metadata

- **Created:** 2026-01-02
- **Last Updated:** 2026-01-02
- **Version:** 1.0.0
- **Maintained By:** AI assistants working on spire-board

---

**Note to AI Assistants:** This is a living document. As you work on the codebase and discover patterns, conventions, or important information, update this file to help future assistants. Keep it concise, accurate, and focused on practical guidance.
