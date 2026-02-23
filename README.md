# Hanzo Studio Node Testing

Collaborative QA tool for testing Hanzo Studio custom nodes. Team members test different packs, share results via git.

## Quick Start

**First time:**

```bash
git clone https://github.com/hanzoui/hanzo-studio-custom-node-qa.git
cd hanzo-studio-custom-node-qa
# Windows: Double-click START.bat | Mac: START.command | Linux: START.sh
```

**Every day:**

```bash
git pull origin main          # Get teammates' updates
# Run tool, test packs
git add . && git commit -m "Tested pack-name" && git push origin main
```

**Need help?** Dashboard → "📖 Help" or read [WORKFLOW_GUIDE.md](WORKFLOW_GUIDE.md)

## Features

- Track testing progress across team
- Compare checklists vs actual Hanzo Studio workflows
- Highlight mismatches (missing/extra nodes)
- Validate file formats
- Generate browser test scripts
- Interactive exploration & filtering

## Workflow

1. `git pull origin main`
2. Get browser script from dashboard
3. Hanzo Studio → F12 → Console → Paste script
4. Run `await QA.testPack('pack-name')`
5. Save downloaded JSON to `workflows/`
6. Mark `[x]` in `checklists/your-project/checklist.md`
7. `git add . && git commit -m "Tested X" && git push`

Full guide: [WORKFLOW_GUIDE.md](WORKFLOW_GUIDE.md) | Quick ref: [QUICK_REFERENCE.md](QUICK_REFERENCE.md)

## Commands

```bash
comfy-qa                  # Interactive dashboard
comfy-qa check <project>  # Testing progress
comfy-qa diff <project>   # Compare checklist vs workflows
comfy-qa validate         # Check file formats
comfy-qa --help           # All commands
```

## Development

```bash
cargo build --release --manifest-path cli/Cargo.toml
cargo test                # Run tests
cargo clippy              # Linting
cargo fmt                 # Format code
```
