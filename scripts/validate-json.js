#!/usr/bin/env node
const fs = require('fs')
const path = require('path')

const workflowsDir = path.join(__dirname, '..', 'workflows')
const files = fs.readdirSync(workflowsDir).filter((f) => f.endsWith('.json'))

let failed = false
for (const file of files) {
  const filepath = path.join(workflowsDir, file)
  try {
    JSON.parse(fs.readFileSync(filepath, 'utf8'))
    console.log(`ok: ${file}`)
  } catch (e) {
    console.error(`FAIL: ${file} - ${e.message}`)
    failed = true
  }
}

process.exit(failed ? 1 : 0)
