# Quick Reference Card

## Every Time You Start Working

```bash
git pull origin main
```
**This gets your teammates' latest work. Always do this first!**

## Testing a Pack

1. Double-click `START.bat` (Windows) or `START.command` (Mac)
2. Select "Get browser script" → Copy it
3. Open ComfyUI in browser → Press F12 → Console tab → Paste script
4. Run: `await QA.testPack('pack-name')`
5. Save the downloaded `.json` file to `workflows/` folder
6. Open `checklists/your-project/checklist.md`
7. Find the pack, change `[ ]` to `[x]`
8. Save the file

## Sharing Your Work

```bash
git add .
git commit -m "Tested pack-name: 42 nodes verified"
git push origin main
```

**Now your teammates can see what you tested!**

## If Push Fails

```bash
git pull origin main
# Fix any conflicts if needed (ask for help!)
git push origin main
```

## Essential Git Commands

| Command | What It Does |
|---------|-------------|
| `git pull origin main` | Get everyone's latest work |
| `git status` | See what files you changed |
| `git add .` | Mark all changes to be saved |
| `git commit -m "message"` | Save changes with description |
| `git push origin main` | Share your work with team |

## Tool Commands (From Dashboard)

| Command | What It Does |
|---------|-------------|
| Check testing progress | See how many packs tested, what's left |
| Get browser script | Get JavaScript to paste in browser |
| View all packs | Browse all packs, filter by status |
| Compare with repo | Find mismatches between checklist and workflows |
| Run validation | Check for format errors |
| Sync checklists | ⚠️ Regenerate checklist from workflows (DESTRUCTIVE!) |

## What the "Sync" Command Does

**WARNING:** This overwrites your checklist with fresh data from workflow files.

**When to use:**
- Workflow files are correct
- Checklist has wrong node counts
- You want to start fresh

**What you'll lose:**
- Your `[x]` tested marks
- Any manual edits to checklist

**Always preview first!** (dry run option)

## Need Help?

1. In the tool: Select "📖 Help - How does this work?"
2. Read [WORKFLOW_GUIDE.md](WORKFLOW_GUIDE.md) for detailed instructions
3. Ask a teammate if git conflicts happen

## Common Mistakes

❌ **Don't:** Test without pulling first → You'll miss teammates' work
✅ **Do:** `git pull origin main` at the start of each session

❌ **Don't:** Sync without previewing → You'll lose your `[x]` marks
✅ **Do:** Use "Preview (dry run)" first

❌ **Don't:** Forget to push → Team can't see your work
✅ **Do:** `git push origin main` after each testing session

❌ **Don't:** Commit without a message → Hard to track what changed
✅ **Do:** Write clear messages: "Tested ComfyUI-Lotus: 2 nodes"
