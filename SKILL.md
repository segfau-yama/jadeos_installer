---
name: logos
description: Multi-model when available, otherwise independent multi-pass synthesis for high-stakes decisions. Use when the user asks for Logos, wants multiple independent opinions, or when architecture, security, destructive, or release-critical work would be expensive to get wrong.
---

# Logos

Logos turns one important decision into independent opinions and one grounded final recommendation.

Use it sparingly. It is for high-stakes judgment, not ordinary chat or routine edits.

## When To Use

Use Logos when:

- the user explicitly asks for Logos
- the user wants multiple independent opinions
- a change is architecture-heavy or hard to reverse
- a change affects security, privacy, or data safety
- an action is destructive or irreversible
- release-critical behavior could regress
- being confidently wrong would be expensive

Do not use Logos for:

- simple summaries
- minor edits
- one-off commands
- work that is faster to verify directly than to panel-review

## Default Review Shape

Use this panel unless the user asks for something else:

1. Practical panel: best available local Codex/GPT path focused on implementation and obvious risks.
2. Stress-test panel: deeper pass focused on hidden assumptions, failure modes, and regressions.
3. Third opinion: concise adversarial pass focused on contradictions, simpler options, and missed risks.
4. Judge: strongest available reviewer synthesizing the evidence into one decision.

Treat provider names as examples, not guarantees. Prefer local review first.

If multiple providers are not available, use independent multi-pass review and explicitly say it was not multi-model.

## Workflow

1. Restate the exact decision, plan, or artifact being judged.
2. Gather independent opinions without letting panelists see each other's outputs first.
3. Inspect code, diffs, tests, or behavior before trusting persuasive prose.
4. Synthesize through:
   - consensus
   - contradictions
   - unique insights
   - blind spots
5. Return the final recommendation first, then the audit trail.

## Fallback Rules

- If a requested model is unavailable, use the best equivalent available and say so.
- If only one model is available, do not pretend the review was multi-model.
- If the same model is used multiple times, call it independent multi-pass review.
- If a third opinion cannot be obtained, continue with the available panels and mark it skipped.
- If an external provider is unavailable or not approved, stay local-only.

## Output Shape

Lead with:

```text
Conclusion: ...
```

Then include:

```text
Called models/tools:
Consensus:
Contradictions:
Unique insights:
Blind spots:
Decision:
Verification:
```

For code or system changes, state what was actually run, inspected, or left unverified.

## Safety

- Prefer local-first review.
- Ask before sending code, logs, prompts, or private content to external providers.
- Never reveal tokens, API keys, passwords, or raw credentials.
- Never claim a model or provider reviewed the work unless it actually ran.
- Never present same-model multi-pass output as multi-model review.
- Require explicit approval before destructive, irreversible, paid, or privacy-sensitive actions.

## JadeOS Repository Notes

In this repository, disk and install operations are inherently destructive.

- Surface erase risk clearly.
- Prefer observed validation over confident speculation.
- Call out rollback limits, prerequisites, and recovery assumptions when reviewing installer changes.
