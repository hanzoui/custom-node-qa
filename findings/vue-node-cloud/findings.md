# Findings

# Findings

## ControlAltAI-Nodes

- [ ] Boolean Basic: true/false widget value for 'value' is hidden in Vue node (shows correctly in LiteGraph)
- [ ] Boolean Reverse: true/false widget value for 'value' is hidden in Vue node (shows correctly in LiteGraph)

## comfyui_layerstyle

- [ ] LayerUtility: Boolean: true/false widget value for 'value' is hidden in Vue node (shows correctly in LiteGraph)

## comfyui_controlnet_aux

- [ ] AIO Aux Preprocessor: 'preprocessor' widget defaults to 'none' despite having available options in the dropdown; suggest using 'undefined' (LiteGraph) or 'no available options' (Vue node) only when models are genuinely unavailable, as a 'none' option seems inappropriate here.
- [ ] Preprocessor Selector: 'preprocessor' widget defaults to 'none' despite having available options in the dropdown; suggest using 'undefined' (LiteGraph) or 'no available options' (Vue node) only when models are genuinely unavailable, as a 'none' option seems inappropriate here.

## comfyui-videohelpersuite

- [x] Load Video FFmpeg (Upload) 🎥🅥🅗🅢: some widgets didn't properly migrate to Vue Nodes, showing LiteGraph.

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7925 and https://github.com/Kosinkadink/ComfyUI-VideoHelperSuite/pull/624, also regarding to "showing LiteGraph", this is not issue, this is because VHS implement its own widgets, we just keep display them properly would be good enough

- [x] Load Audio (Upload) 🎥🅥🅗🅢: some widgets didn't properly migrate to Vue Nodes, showing LiteGraph + two audio players are present in the node (one for Vue node, one for LiteGraph), if resize node to be longer vertically.. the incorrectly migrated old audio player doesnt resize.. and instead extends out of the node itself.

  Terry - should fix in https://github.com/Kosinkadink/ComfyUI-VideoHelperSuite/pull/626

- [x] Meta Batch Manager 🎥🅥🅗🅢: some widgets didn't properly migrate to Vue Nodes, showing LiteGraph.

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7925 and https://github.com/Kosinkadink/ComfyUI-VideoHelperSuite/pull/624, also regarding to "showing LiteGraph", this is not issue, this is because VHS implement its own widgets, we just keep display them properly would be good enough

- [x] VAE Encode Batched 🎥🅥🅗🅢: some widgets didn't properly migrate to Vue Nodes, showing LiteGraph.

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7925 and https://github.com/Kosinkadink/ComfyUI-VideoHelperSuite/pull/624, also regarding to "showing LiteGraph", this is not issue, this is because VHS implement its own widgets, we just keep display them properly would be good enough

- [x] VAE Decode Batched 🎥🅥🅗🅢: some widgets didn't properly migrate to Vue Nodes, showing LiteGraph.

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7925 and https://github.com/Kosinkadink/ComfyUI-VideoHelperSuite/pull/624, also regarding to "showing LiteGraph", this is not issue, this is because VHS implement its own widgets, we just keep display them properly would be good enough

- [x] Split Latents 🎥🅥🅗🅢: some widgets didn't properly migrate to Vue Nodes, showing LiteGraph.

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7925 and https://github.com/Kosinkadink/ComfyUI-VideoHelperSuite/pull/624, also regarding to "showing LiteGraph", this is not issue, this is because VHS implement its own widgets, we just keep display them properly would be good enough

- [x] Split Masks 🎥🅥🅗🅢: some widgets didn't properly migrate to Vue Nodes, showing LiteGraph.

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7925 and https://github.com/Kosinkadink/ComfyUI-VideoHelperSuite/pull/624, also regarding to "showing LiteGraph", this is not issue, this is because VHS implement its own widgets, we just keep display them properly would be good enough

- [x] Repeat Latents 🎥🅥🅗🅢: some widgets didn't properly migrate to Vue Nodes, showing LiteGraph.

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7925 and https://github.com/Kosinkadink/ComfyUI-VideoHelperSuite/pull/624, also regarding to "showing LiteGraph", this is not issue, this is because VHS implement its own widgets, we just keep display them properly would be good enough

- [x] Repeat Masks 🎥🅥🅗🅢: some widgets didn't properly migrate to Vue Nodes, showing LiteGraph.

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7925 and https://github.com/Kosinkadink/ComfyUI-VideoHelperSuite/pull/624, also regarding to "showing LiteGraph", this is not issue, this is because VHS implement its own widgets, we just keep display them properly would be good enough

- [x] Select Every Nth Latent 🎥🅥🅗🅢: some widgets didn't properly migrate to Vue Nodes, showing LiteGraph.

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7925 and https://github.com/Kosinkadink/ComfyUI-VideoHelperSuite/pull/624, also regarding to "showing LiteGraph", this is not issue, this is because VHS implement its own widgets, we just keep display them properly would be good enough

- [x] Select Every Nth Mask 🎥🅥🅗🅢: some widgets didn't properly migrate to Vue Nodes, showing LiteGraph.

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7925 and https://github.com/Kosinkadink/ComfyUI-VideoHelperSuite/pull/624, also regarding to "showing LiteGraph", this is not issue, this is because VHS implement its own widgets, we just keep display them properly would be good enough

- [x] Select Latest 🎥🅥🅗🅢: some widgets didn't properly migrate to Vue Nodes, showing LiteGraph.

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7925 and https://github.com/Kosinkadink/ComfyUI-VideoHelperSuite/pull/624, also regarding to "showing LiteGraph", this is not issue, this is because VHS implement its own widgets, we just keep display them properly would be good enough

  ## comfyui-animatediff-evolved

- [x] Load CameraCtrl Poses (Path) 🎭🅐🅓②: cannot retrieve file path.

  Terry - This path uses Path(file_path), an absolute path, which is designed for local environments; tbd about disabling.

## comfyui-impact-pack

- [x] SEGSOrderedFilterDetailerHookProvider: ascending/descending widget value for 'order' is hidden in Vue node (shows correctly in LiteGraph)

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7894

- [x] SEGSRangeFilterDetailerHookProvider: inside/outside widget value for 'mode' is hidden in Vue node (shows correctly in LiteGraph)

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7894

- [x] Switch (latent/legacy): select_on_execution/select_on_prompt widget value for 'set_mode' is hidden in Vue node (shows correctly in LiteGraph)

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7894

- [x] Switch (SEGS/legacy): select_on_execution/select_on_prompt widget value for 'set_mode' is hidden in Vue node (shows correctly in LiteGraph)

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7894

- [x] Switch (Any): select_on_execution/select_on_prompt widget value for 'set_mode' is hidden in Vue node (shows correctly in LiteGraph)

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7894

- [x] Inversed Switch (Any): select_on_execution/select_on_prompt widget value for 'set_mode' is hidden in Vue node (shows correctly in LiteGraph)

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7894

- [x] SEGSDetailer For Video (SEGS/pipe): bbox/crop_region widget value for 'guide_size_for' is hidden in Vue node (shows correctly in LiteGraph)

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7894

- [x] SEGS Filter (range): inside/outside widget value for 'mode' is hidden in Vue node (shows correctly in LiteGraph)

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7894

- [x] SEGS Filter (ordered): inside/outside widget value for 'mode' is hidden in Vue node (shows correctly in LiteGraph)

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7894

- [x] ImpactMinMax: max/min widget value for 'mode' is hidden in Vue node (shows correctly in LiteGraph)

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7894

- [x] Control Bridge: active/(stop/mute/bypass) widget value for 'mode' is hidden in Vue node (shows correctly in LiteGraph)

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7894

- [x] Remote Boolean (on prompt): true/false widget value for 'value' is hidden in Vue node (shows correctly in LiteGraph)

  Terry - should fix in https://github.com/Comfy-Org/ComfyUI_frontend/pull/7894

  ## comfyui-kjnodes

- [ ] Conditioning Multi Combine: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to LiteGraph --> back to Vue node)
- [ ] Mask Batch Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to LiteGraph --> back to Vue node)
- [ ] Cross Fade Images Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to LiteGraph --> back to Vue node)
- [ ] Image Add Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to LiteGraph --> back to Vue node)
- [ ] Image Batch Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to LiteGraph --> back to Vue node)
- [ ] Image Concatenate Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to LiteGraph --> back to Vue node)
- [ ] Transition Images Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to LiteGraph --> back to Vue node)
- [ ] Join String Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to LiteGraph --> back to Vue node)
- [ ] Empty Latent Image Custom Presets: no presets available from drop-down, seems unintuitive.
- [x] Spline Editor: unable to interact with the node UI using Vue node (works correctly in LiteGraph)

  Terry - there are several issues for this node, need to fix one by one:
  - resize - https://github.com/Comfy-Org/ComfyUI_frontend/pull/7934
  - unexpected node dragable - https://github.com/Comfy-Org/ComfyUI_frontend/pull/7953
  - protovis coordinate issue requires both https://github.com/Comfy-Org/ComfyUI_frontend/pull/7963 and https://github.com/kijai/ComfyUI-KJNodes/pull/497

- [x] Points Editor: unable to interact with the node UI using Vue node (works correctly in LiteGraph)

  Terry - there are several issues for this node, need to fix one by one:
  - resize - https://github.com/Comfy-Org/ComfyUI_frontend/pull/7934
  - unexpected node dragable - https://github.com/Comfy-Org/ComfyUI_frontend/pull/7953
  - protovis coordinate issue requires both https://github.com/Comfy-Org/ComfyUI_frontend/pull/7963 and https://github.com/kijai/ComfyUI-KJNodes/pull/497

- [ ] GGUFLoaderKJ: dropdown shows no model options.

  ## comfy_extras

- [ ] Record Audio: difficult to use, unable to stop/start recording effectively + recording audio ui on node extends out of the node itself for Vue node.
- [ ] Load 3D & Animation: uploaded file appears with random numbers/letters for filename on node.
- [ ] Boolean: true/false widget value for 'value' is hidden in Vue node (shows correctly in LiteGraph)
- [ ] Preview as Text: markdown/plaintext widget value for 'previewmode' is hidden in Vue node (shows correctly in LiteGraph)

  ## misc

- [ ] collapsed nodes extend in size when switching to LiteGraph --> back to Vue node
- [ ] collapsed nodes seem to have the correct width for the node (so it shows the node name + custom node name), could be used as a base to resize all nodes correctly once loaded in?
- [x] unable to set seed to a 'fixed' option with the options available in the new seed layout for Vue node.

  Austin - Fixed

