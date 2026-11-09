# The Motivation Behind PRYSM

## The Paradigm Shift

Git was engineered in 2005 for a specific mental model: **one human working sequentially on a single codebase.** 

Even with modern branching models (Gitflow, Trunk-Based Development), the assumption remains that code is modified linearly, one change at a time, with humans resolving line-by-line conflicts manually at the merge boundary.

Today, development looks completely different.

## The AI Agent Reality

We have entered the era of AI-assisted and autonomous development. A single developer now routinely orchestrates multiple agents in parallel:
- Agent A refactors the API layer.
- Agent B writes unit tests for authentication.
- Agent C builds the UI components.

When you force multiple agents or concurrent workflows into a single working directory, everything breaks:
1. **File Overwrites:** Agents overwrite each other's intermediate state.
2. **Context Pollution:** Terminal outputs, build artifacts, and partial edits interleave.
3. **Merge Blindness:** Standard Git merges only look at *what lines changed*, completely ignoring *why the change was made*.

## Why Existing Tools Fall Short

Standard Git commands (`git checkout`, `git branch`, `git stash`) force context-switching within the same workspace.

While Git has supported `git worktree` for years, its native command-line interface is notoriously hostile, error-prone, and lacks any concept of metadata, agent identification, or semantic safety boundaries.

We don't need a cloud dashboard or a proprietary SaaS platform to manage this. We need a **local, Git-native orchestrator**.

## Enter PRYSM

PRYSM treats concurrency not as an afterthought, but as the foundational architecture.

- **Isolated Realities:** Every agent gets its own pristine, independent worktree instantly.
- **Intent-Aware Convergence™:** By attaching a lightweight intent metadata file to each reality, PRYSM tracks *intent* and *scope* rather than just diffs.
- **Zero Cloud, 100% Local:** Your code stays on your machine. No telemetry, no third-party servers, no latency.

Concurrency isn't a feature. It's the architecture of the future. PRYSM makes it native.
