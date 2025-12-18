# ComfyUI Custom Node QA

Tools and artifacts for QA testing custom nodes on ComfyUI Cloud.

## Scripts

`scripts/qa.js` - Browser devtools utilities for batch testing node packs.

Usage: paste into browser console, then:
```js
await QA.listPacks()                    // list all packs
await QA.checklist()                    // markdown checklist -> clipboard
await QA.testPack("comfyui-impact-pack") // clear, add all nodes, save
await QA.testAllPacks()                 // test every pack
QA.help()                               // full command list
```

## Workflows

`workflows/` contains saved workflow files with all nodes from each pack. Naming: `all-nodes-{pack-name}.json`

## Results

`results/` - store QA session results here. Copy `CHECKLIST_TEMPLATE.md` to `results/YYYY-MM-DD.md` for each session.

## Adding new scripts

Add new devtools scripts to `scripts/`. Keep them self-contained and paste-able into browser console.
