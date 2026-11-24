---
name: feedback-commit-timing
description: Don't commit after every small change — wait for the user to say when to commit
metadata:
  type: feedback
---

Don't propose or run a commit after each individual change. The user will say when they're ready to commit.

**Why:** User explicitly asked to stop — they want to batch changes and control when commits happen.

**How to apply:** Make the requested edits silently. Only commit when the user explicitly says to (e.g. "commit", "push", "that's good").
