# QA Checklist Template

Copy this file to `results/YYYY-MM-DD.md` when starting a QA session.

Generate pack list with `await QA.checklist()` in devtools.

## Test procedure per pack

1. Load workflow from `workflows/all-nodes-{pack}.json`
2. Verify all nodes instantiate without errors
3. Check node UIs render correctly
4. Note any console errors
5. Mark pack as done below

## Results

(paste output from QA.checklist() here)
