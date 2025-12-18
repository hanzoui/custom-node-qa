/**
 * ComfyUI QA DevTools - paste into console
 * QA.listPacks() | QA.testPack("comfyui-impact-pack") | QA.help()
 */
const QA = {
  _defs: null,
  _byMod: null,

  _normalizeMod(mod) {
    if (mod.startsWith("comfy_extras.")) return "comfy_extras"
    if (mod.startsWith("comfy_api_nodes.")) return "comfy_api_nodes"
    if (mod.startsWith("custom_nodes.")) return mod.replace("custom_nodes.", "")
    if (mod === "nodes") return "core"
    return mod
  },

  async _init() {
    if (!this._defs) {
      this._defs = await app.getNodeDefs()
      this._byMod = {}
      for (const [name, def] of Object.entries(this._defs)) {
        const mod = this._normalizeMod(def.python_module || "core")
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

  async checklist() {
    await this._init()
    const lines = Object.keys(this._byMod).sort().map(mod =>
      `- [ ] ${mod} (${this._byMod[mod].length})`
    )
    const md = "# Node Pack QA Checklist\n\n" + lines.join("\n")
    await navigator.clipboard.writeText(md)
    return md
  },

  async detailedChecklist() {
    await this._init()
    const sections = Object.keys(this._byMod).sort().map(mod => {
      const nodes = this._byMod[mod].map(n =>
        `- [ ] ${n.display_name || n.name}${n.deprecated ? " ~~DEPRECATED~~" : ""}`
      )
      return `## ${mod}\n\n${nodes.join("\n")}`
    })
    const md = "# Node Pack QA Checklist\n\n" + sections.join("\n\n")
    await navigator.clipboard.writeText(md)
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
    const nodes = (this._byMod[mod] || []).filter(n => !skip.includes(n.name) && !n.deprecated)

    return nodes.map((def, i) => {
      try {
        return this.addNode(def.name, [100 + (i % cols) * spacing[0], 100 + Math.floor(i / cols) * spacing[1]])
      } catch { return null }
    }).filter(Boolean)
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
    await new Promise(r => setTimeout(r, 300))
    const filename = `all-nodes-${mod.replace(/[^a-z0-9_-]/gi, '_')}`
    if (opts.save !== false) await this.save(filename)
    return nodes
  },

  async testPacks(mods, opts = {}) {
    for (const mod of mods) {
      await this.testPack(mod, opts)
      await new Promise(r => setTimeout(r, 1000))
    }
  },

  async testAllPacks(opts = {}) {
    await this._init()
    const mods = Object.keys(this._byMod).sort()
    for (const mod of mods) {
      await this.testPack(mod, opts)
      await new Promise(r => setTimeout(r, 1000))
    }
  },

  help() {
    console.log(`
QA.listPacks()              - list all packs
QA.checklist()              - markdown checklist (copies to clipboard)
QA.detailedChecklist()      - with individual nodes
QA.testPack("comfyui-impact-pack")  - clear, add all, save
QA.testPacks([...])         - test multiple
QA.testAllPacks()           - test every pack
QA.addNode("KSampler")      - add single node
QA.addPack("mod")           - add all from pack
QA.save("name")             - save workflow
`)
  }
}
