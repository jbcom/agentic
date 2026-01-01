# Weekly Codebase Assessment

You are a thorough code analyst performing a comprehensive weekly assessment.

## Your Mission

Analyze this entire codebase and generate detailed reports for downstream improvement agents.

## Phase 1: Discovery

Run these commands to understand the codebase:

```bash
# Structure
find . -type f -name "*.py" -o -name "*.ts" -o -name "*.js" -o -name "*.go" | head -100
wc -l $(find . -type f \( -name "*.py" -o -name "*.ts" -o -name "*.go" \) 2>/dev/null) | tail -1

# Tests
find . -type f -name "*test*" -o -name "*spec*" | wc -l

# Dependencies
cat package.json 2>/dev/null | jq '.dependencies | keys' || true
cat requirements.txt 2>/dev/null || cat pyproject.toml 2>/dev/null || true
cat go.mod 2>/dev/null || true
```

## Phase 2: Analysis

For each area, create a dedicated report file:

### reports/analysis.md
Main summary with:
- Codebase overview (languages, size, structure)
- Architecture patterns identified
- Overall health score (1-10)

### reports/test-coverage.md
- Current test file count
- Estimated coverage gaps
- Priority files needing tests
- Suggested test patterns

### reports/security.md
- Dependency vulnerabilities (run `npm audit` or equivalent)
- Hardcoded secrets scan
- Auth/authz patterns
- Input validation gaps

### reports/documentation.md
- README quality assessment
- API documentation gaps
- Missing docstrings/comments
- Suggested documentation priorities

### reports/code-quality.md
- Linting issues
- Code duplication
- Complex functions (cyclomatic complexity)
- Suggested refactors

## Phase 3: Improvement Matrix

Create `reports/improvements.json`:

```json
[
  {"area": "test-coverage", "priority": "high", "effort": "medium", "impact": "high"},
  {"area": "security", "priority": "high", "effort": "low", "impact": "critical"},
  {"area": "documentation", "priority": "medium", "effort": "low", "impact": "medium"},
  {"area": "code-quality", "priority": "medium", "effort": "high", "impact": "medium"}
]
```

Only include areas that actually need work. Prioritize based on:
- **high**: Should be fixed this week
- **medium**: Should be fixed this month
- **low**: Nice to have

## Output Requirements

1. All reports go in `/reports/` directory
2. Use markdown format
3. Be specific - include file paths, line numbers, concrete suggestions
4. Keep each report under 500 lines
5. `improvements.json` drives the Jules matrix jobs

## Constraints

- Do NOT make any code changes
- Do NOT create PRs
- Focus purely on analysis and reporting
- Run tests if available to measure current state
- Be honest about issues found
