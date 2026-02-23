# Git Automation Research

## Question: Should we automate git commands in the CLI?

### Options Evaluated

#### Option 1: Full Automation

**Auto-run:**

- `git pull` on app startup
- `git add/commit/push` after marking tests complete
- `git status` checks before operations

**Pros:**

- Seamless experience for non-technical users
- No need to remember git commands
- Always in sync with team

**Cons:**

- Dangerous: Could commit/push unintended changes
- Users lose control over what gets committed
- Merge conflicts become scary/confusing
- Can't review changes before committing
- Bad commit messages (auto-generated)
- Violates git best practices

**Verdict:** ❌ Too risky, violates user control

#### Option 2: Prompted Automation

**Ask before each operation:**

- "Pull latest changes from team? [Y/n]"
- "Commit your changes? [Y/n]"
- "Push to share with team? [Y/n]"

**Pros:**

- User has control
- Still easier than manual git
- Can add good commit messages

**Cons:**

- Annoying prompts every time
- Users might blindly hit Yes
- Still risky if user doesn't understand
- Interrupts workflow

**Verdict:** ❌ Annoying and still risky

#### Option 3: Helper Commands (RECOMMENDED)

**Add git helper commands to dashboard:**

- "Pull latest from team" → runs `git pull`
- "Show what changed" → runs `git status` and `git diff`
- "Commit and push my work" → Interactive flow with commit message

**Pros:**

- Users can choose when to sync
- Still provides easy access
- Safe: explicit user action
- Can add helpful prompts/guidance
- Checks for issues before running
- Users learn what's happening

**Cons:**

- Not fully automatic
- Users must remember to sync

**Verdict:** ✅ Best balance of safety and ease

#### Option 4: Status Only (Minimal)

**Just show git status:**

- Dashboard shows: "3 uncommitted changes"
- "Behind by 5 commits (pull to sync)"
- Links to instructions but doesn't run commands

**Pros:**

- Very safe
- Users maintain full control
- No risk of automation errors

**Cons:**

- Least helpful for non-technical users
- Still requires manual git commands
- Doesn't solve the original problem

**Verdict:** ❌ Too minimal, doesn't help enough

### Recommended Implementation: Option 3

Add three new dashboard options:

1. **"Pull latest from team"**

   ```
   - Runs: git pull origin main
   - Shows summary of what changed
   - Handles errors gracefully
   - Suggests resolving conflicts if needed
   ```

2. **"Review my changes"**

   ```
   - Runs: git status and git diff --stat
   - Shows files changed
   - Preview what would be committed
   ```

3. **"Commit and share my work"**
   ```
   - Interactive flow:
     1. Show git status (what changed)
     2. Ask for commit message
     3. Suggest good message based on changes
     4. Run: git add .
     5. Run: git commit -m "message"
     6. Ask: Push now?
     7. Run: git push origin main
     8. Handle errors (e.g., need to pull first)
   ```

### Safety Features

- **Check git is installed** before offering options
- **Check in git repo** before running commands
- **Check for conflicts** after pull
- **Show output** of all git commands
- **Handle errors** gracefully with helpful messages
- **Dry run option** to preview
- **Undo guidance** if something goes wrong

### Error Handling

Common scenarios:

**Push rejected (need to pull first):**

```
❌ Push failed: Remote has new changes
→ Run "Pull latest from team" first
→ Resolve any conflicts
→ Try push again
```

**Merge conflicts:**

```
⚠️ Merge conflict in checklist.md
→ Manual fix needed
→ See WORKFLOW_GUIDE.md for conflict resolution
→ Or ask a teammate for help
```

**Uncommitted changes before pull:**

```
⚠️ You have uncommitted changes
Options:
  1. Commit them first (recommended)
  2. Stash them temporarily
  3. Cancel pull
```

### Dependencies

Need to add:

```toml
[dependencies]
git2 = "0.18"  # Rust git library
```

OR just shell out to git command:

```rust
std::process::Command::new("git")
    .args(&["pull", "origin", "main"])
    .status()?;
```

**Recommendation:** Shell out to git command (simpler, users need git installed anyway)

### Implementation Plan

1. Add helper functions in `cli/src/git.rs`:
   - `check_git_installed()`
   - `check_in_repo()`
   - `git_pull()`
   - `git_status()`
   - `git_commit_and_push(interactive: bool)`

2. Add to dashboard menu after "Generate workflow":

   ```
   "🔄 Pull latest from team"
   "📝 Review my changes"
   "📤 Commit and share my work"
   ```

3. Add help topic in guide explaining these features

4. Update WORKFLOW_GUIDE.md with new workflow:
   ```
   Option A: Use CLI git helpers (easier)
   Option B: Use git commands directly (more control)
   ```

### User Stories

**Story 1: Non-technical tester**

- Opens app, sees "5 commits behind"
- Clicks "Pull latest from team"
- Sees summary: "3 new packs tested by Alice"
- Continues testing
- Clicks "Commit and share my work"
- Enters message: "Tested Hanzo Studio-Lotus"
- App commits and pushes automatically
- Success message with what was shared

**Story 2: Technical tester**

- Uses git commands directly in terminal
- App doesn't interfere
- Helper options available but not required
- Both workflows coexist

**Story 3: Conflict scenario**

- Clicks "Pull latest"
- App detects merge conflict
- Shows clear message
- Links to conflict resolution guide
- Doesn't try to auto-resolve (dangerous)

### Conclusion

**Implement Option 3: Helper Commands**

- Add `git.rs` module with safe wrappers
- Add 3 dashboard options for common operations
- Shell out to git command (don't use git2 library)
- Extensive error handling and guidance
- Never auto-run without explicit user action
- Make it optional (users can still use terminal)

This provides the best balance:

- ✅ Easier for non-technical users
- ✅ Safe and explicit
- ✅ Users learn what's happening
- ✅ Coexists with manual git usage
- ✅ Handles errors gracefully
