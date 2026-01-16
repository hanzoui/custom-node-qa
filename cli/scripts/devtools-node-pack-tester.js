/**
 * Copy/paste this entire file into browser devtools console.
 * Then: QA.listPacks() | QA.testPack("comfyui-impact-pack") | QA.help()
 */
const QA = {
  _defs: null,
  _byMod: null,

  _normalizeMod(mod) {
    if (mod.startsWith('comfy_extras.')) return 'comfy_extras'
    if (mod.startsWith('comfy_api_nodes.')) return 'comfy_api_nodes'
    if (mod.startsWith('custom_nodes.')) return mod.replace('custom_nodes.', '')
    if (mod === 'nodes') return 'core'
    return mod
  },

  async _init() {
    if (!this._defs) {
      this._defs = await app.getNodeDefs()
      this._byMod = {}
      for (const [name, def] of Object.entries(this._defs)) {
        const mod = this._normalizeMod(def.python_module || 'core')
        ;(this._byMod[mod] ??= []).push({ name, ...def })
      }
    }
  },

  async listPacks() {
    await this._init()
    return Object.entries(this._byMod)
      .map(([mod, nodes]) => `${mod}: ${nodes.length}`)
      .sort()
  },

  async checklist(filename = 'checklist.md') {
    await this._init()
    const lines = Object.keys(this._byMod)
      .sort()
      .map((mod) => `- [ ] ${mod} (${this._byMod[mod].length})`)
    const md = '# Node Pack QA Checklist\n\n' + lines.join('\n')
    this._download(md, filename)
    return md
  },

  async detailedChecklist(filename = 'checklist-detailed.md') {
    await this._init()
    const sections = Object.keys(this._byMod)
      .sort()
      .map((mod) => {
        const nodes = this._byMod[mod].map(
          (n) => `- [ ] ${n.display_name || n.name}${n.deprecated ? ' ~~DEPRECATED~~' : ''}`
        )
        return `## ${mod}\n\n${nodes.join('\n')}`
      })
    const md = '# Node Pack QA Checklist\n\n' + sections.join('\n\n')
    this._download(md, filename)
    return md
  },

  addNode(type, pos) {
    const node = LiteGraph.createNode(type, null, { pos })
    app.graph.add(node)
    return node
  },

  async addPack(mod, opts = {}) {
    await this._init()
    const { cols = 5, spacing = [400, 300], skip = [] } = opts
    const nodes = (this._byMod[mod] || []).filter((n) => !skip.includes(n.name) && !n.deprecated)

    return nodes
      .map((def, i) => {
        try {
          return this.addNode(def.name, [
            100 + (i % cols) * spacing[0],
            100 + Math.floor(i / cols) * spacing[1]
          ])
        } catch {
          return null
        }
      })
      .filter(Boolean)
  },

  _download(content, filename, type = 'text/plain') {
    const blob = new Blob([content], { type })
    const a = Object.assign(document.createElement('a'), {
      href: URL.createObjectURL(blob),
      download: filename
    })
    a.click()
  },

  async save(name) {
    this._download(JSON.stringify(app.graph.serialize()), `${name}.json`, 'application/json')
  },

  async testPack(mod, opts = {}) {
    app.graph.clear()
    const nodes = await this.addPack(mod, opts)
    if (!nodes.length) return []
    await new Promise((r) => setTimeout(r, 300))
    const filename = `all-nodes-${mod.replace(/[^a-z0-9_-]/gi, '_')}`
    if (opts.save !== false) await this.save(filename)
    return nodes
  },

  async testPacks(mods, opts = {}) {
    for (const mod of mods) {
      await this.testPack(mod, opts)
      await new Promise((r) => setTimeout(r, 1000))
    }
  },

  async testAllPacks(opts = {}) {
    await this._init()
    const mods = Object.keys(this._byMod).sort()
    for (const mod of mods) {
      await this.testPack(mod, opts)
      await new Promise((r) => setTimeout(r, 1000))
    }
  },

  /**
   * Export current state as JSON for CLI import
   */
  async export(projectName = 'exported-state') {
    await this._init()
    const data = {
      version: '1.0',
      exported_at: new Date().toISOString(),
      project_name: projectName,
      environment: {
        url: window.location.origin,
        user_agent: navigator.userAgent,
        comfyui_version: window.comfyAPI?.version || 'unknown'
      },
      packs: Object.entries(this._byMod).map(([mod, nodes]) => ({
        name: mod,
        node_count: nodes.length,
        nodes: nodes.map(n => ({
          name: n.name,
          display_name: n.display_name,
          deprecated: !!n.deprecated,
          category: n.category
        }))
      }))
    }
    this._download(JSON.stringify(data, null, 2), `${projectName}-export.json`, 'application/json')
    console.log(`✅ Exported ${Object.keys(this._byMod).length} packs to ${projectName}-export.json`)
    console.log(`📋 Import to CLI: comfy-qa import ${projectName}-export.json ${projectName}`)
    return data
  },

  /**
   * Generate copy-paste snippet for GitHub issue
   */
  async snippet(projectName) {
    await this._init()
    const stats = {
      total_packs: Object.keys(this._byMod).length,
      total_nodes: Object.values(this._byMod).reduce((sum, nodes) => sum + nodes.length, 0),
      environment: window.location.origin,
      timestamp: new Date().toISOString()
    }
    const markdown = `
## QA Environment Snapshot

**Project:** ${projectName}
**Environment:** ${stats.environment}
**Timestamp:** ${stats.timestamp}

**Node Packs Available:** ${stats.total_packs}
**Total Nodes:** ${stats.total_nodes}

<details>
<summary>Pack List</summary>

${Object.entries(this._byMod)
  .sort()
  .map(([mod, nodes]) => `- ${mod} (${nodes.length})`)
  .join('\n')}

</details>
`
    console.log(markdown)
    navigator.clipboard.writeText(markdown)
    console.log('📋 Copied to clipboard! Paste into GitHub issue.')
    return markdown
  },

  /**
   * Quick diff against repo checklist (fetches from GitHub)
   */
  async diff(projectName, branch = 'main') {
    await this._init()
    const repoUrl = `https://raw.githubusercontent.com/Comfy-Org/comfyui-custom-node-qa/${branch}/checklists/${projectName}/checklist.md`

    try {
      const response = await fetch(repoUrl)
      if (!response.ok) throw new Error(`Checklist not found: ${projectName}`)
      const markdown = await response.text()

      // Parse checklist
      const checklistPacks = new Map()
      const lines = markdown.split('\n')
      for (const line of lines) {
        const match = line.match(/- \[([ x])\] (.+?) \((\d+)\)/)
        if (match) {
          const [, checked, name, count] = match
          checklistPacks.set(name, { checked: checked === 'x', count: parseInt(count) })
        }
      }

      // Compare
      const live = new Set(Object.keys(this._byMod))
      const checklist = new Set(checklistPacks.keys())

      const results = {
        matches: [],
        count_mismatch: [],
        missing_workflow: [],
        new_packs: [],
        untested: []
      }

      for (const pack of live) {
        if (!checklist.has(pack)) {
          results.new_packs.push({ pack, count: this._byMod[pack].length })
        } else {
          const cl = checklistPacks.get(pack)
          const liveCount = this._byMod[pack].length
          if (cl.count !== liveCount) {
            results.count_mismatch.push({ pack, checklist: cl.count, live: liveCount })
          } else if (!cl.checked) {
            results.untested.push({ pack, count: liveCount })
          } else {
            results.matches.push({ pack, count: liveCount })
          }
        }
      }

      for (const pack of checklist) {
        if (!live.has(pack)) {
          results.missing_workflow.push({ pack, count: checklistPacks.get(pack).count })
        }
      }

      // Pretty print
      console.log(`\n📊 Diff: ${projectName} (${branch})`)
      console.log(`\n✅ Matches: ${results.matches.length}`)
      console.log(`⚠️  Count Mismatches: ${results.count_mismatch.length}`)
      if (results.count_mismatch.length) {
        results.count_mismatch.forEach(({pack, checklist, live}) => {
          console.log(`   • ${pack}: checklist=${checklist}, live=${live} (${live > checklist ? '+' : ''}${live - checklist})`)
        })
      }
      console.log(`❌ Missing in Environment: ${results.missing_workflow.length}`)
      if (results.missing_workflow.length) {
        results.missing_workflow.forEach(({pack}) => console.log(`   • ${pack}`))
      }
      console.log(`🆕 New Packs (not in checklist): ${results.new_packs.length}`)
      if (results.new_packs.length) {
        results.new_packs.forEach(({pack, count}) => console.log(`   • ${pack} (${count})`))
      }
      console.log(`⏳ Untested (in checklist, unchecked): ${results.untested.length}`)

      return results

    } catch (err) {
      console.error(`❌ Error fetching checklist: ${err.message}`)
      console.log(`\n💡 Tip: First run QA.export('${projectName}') to create initial state`)
      return null
    }
  },

  /**
   * List available projects from repo
   */
  async projects(branch = 'main') {
    const url = `https://api.github.com/repos/Comfy-Org/comfyui-custom-node-qa/contents/checklists?ref=${branch}`
    try {
      const response = await fetch(url)
      const dirs = await response.json()
      const projects = dirs
        .filter(d => d.type === 'dir' && !['templates', 'schema'].includes(d.name))
        .map(d => d.name)

      console.log(`\n📁 Available Projects (${projects.length}):`)
      projects.forEach(p => console.log(`   • ${p}`))
      console.log(`\n💡 Run QA.diff('project-name') to compare`)
      return projects
    } catch (err) {
      console.error(`❌ Error fetching projects: ${err.message}`)
      return []
    }
  },

  help() {
    console.log(`
QA Commands:

  Basic:
    QA.listPacks()              - list all packs in current environment
    QA.checklist()              - download markdown checklist
    QA.detailedChecklist()      - download with individual nodes
    QA.testPack("pack-name")    - clear, add all nodes, save workflow
    QA.testAllPacks()           - test every pack

  Integration with CLI:
    QA.export("project-name")   - export current state as JSON
    QA.diff("project-name")     - compare live vs repo checklist
    QA.snippet("project-name")  - generate GitHub issue snippet
    QA.projects()               - list available projects in repo

  Advanced:
    QA.addNode("KSampler")      - add single node
    QA.addPack("mod")           - add all nodes from pack
    QA.save("filename")         - save current workflow
`)
  }
}
