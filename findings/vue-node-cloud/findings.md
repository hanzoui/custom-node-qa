# Findings

## ComfyUI-WanVideoWrapper
- WanVideo Tiny VAE Loader: dropdown shows no model options.
- Wav2vec2 Model Loader: dropdown shows no model options.

## CustomNodeComfyMath
- BoolToInt: true/false widget value is hidden in Vue node (shows correctly in legacy node layout).
- BoolUnaryOperation: true/false widget value is hidden in Vue node (shows correctly in legacy node layout).
- BoolBinaryOperation: true/false widget value is hidden in Vue node (shows correctly in legacy node layout).

## audio-separation-nodes-comfyui
- AudioVideoCombine: cannot retrieve video path or upload a video via the widget.

## comfy_api_nodes
- Flux 1.1 [pro] Ultra Image: aspect ratio must be typed manually; no preset dropdown.
- Flux.1 Kontext [pro] Image: aspect ratio must be typed manually; no preset dropdown.
- Flux.1 Kontext [max] Image: aspect ratio must be typed manually; no preset dropdown.
- Recraft Style - Infinite Style Library: unclear how to retrieve or use a style_id; node unusable as-is.
- Stability AI Text To Audio: duration entry only accepts frames; seconds input would help.
- Stability AI Audio To Audio: duration entry only accepts frames; seconds input would help.
- Stability AI Audio Inpaint: duration entry only accepts frames; seconds input would help.
- Tripo: Text to Model: seed button layout/controls missing in Vue node.
- Tripo: Image to Model: seed button layout/controls missing in Vue node.
- Tripo: Multiview to Model: seed button layout/controls missing in Vue node.
- Tripo: Texture model: seed button layout/controls missing in Vue node.
- Moonvalley Marey Video to Video: seed button layout/controls missing in Vue node.
- Rodin 3D Generate - Regular Generate: seed button layout/controls missing in Vue node.
- Rodin 3D Generate - Detail Generate: seed button layout/controls missing in Vue node.
- Rodin 3D Generate - Smooth Generate: seed button layout/controls missing in Vue node.
- Rodin 3D Generate - Sketch Generate: seed button layout/controls missing in Vue node.
- Rodin 3D Generate - Gen-2 Generate: seed button layout/controls missing in Vue node.

## core
- Load Image (from Outputs): if the most recent output was a video, it attempts to load that and "Image failed to load" error is present on the node.
- DiffusersLoader: dropdown shows no model options.
- LoadLatent: unable to load latent despite saving a latent using 'SaveLatent' node.

## comfyui_ipadapter_plus
-IPAdapter Load Embeds: dropdown shows no model options.

## comfyui_essentials
- 🔧 Flux Sampler Parameters: 'noise' widget classified as STRING, expected to be an INT for a seed - loads in with "?".
- 🔧 Simple Math: 'a', 'b' inputs are hard to notice visually - too transparent.

## comfyui_controlnet_aux
- AIO Aux Preprocessor: 'preprocessor' widget defaults to 'none' despite having available options in the dropdown; suggest using 'undefined' (legacy layout) or 'no available options' (Vue node) only when models are genuinely unavailable, as a 'none' option seems inappropriate here.
- Preprocessor Selector: 'preprocessor' widget defaults to 'none' despite having available options in the dropdown; suggest using 'undefined' (legacy layout) or 'no available options' (Vue node) only when models are genuinely unavailable, as a 'none' option seems inappropriate here.

## comfyui-videohelpersuite
- Load Video FFmpeg (Upload) 🎥🅥🅗🅢: some widgets didn’t properly migrate to Vue Nodes, showing the old node layout.
- Load Audio (Upload)🎥🅥🅗🅢: some widgets didn’t properly migrate to Vue Nodes, showing the old node layout + two audio players are present in the node (one for Vue node, one for legacy node layout), if resize node to be longer vertically.. the incorrectly migrated old audio player doesnt resize.. and instead extends out of the node itself.
- Meta Batch Manager 🎥🅥🅗🅢: some widgets didn’t properly migrate to Vue Nodes, showing the old node layout.
- VAE Encode Batched 🎥🅥🅗🅢: some widgets didn’t properly migrate to Vue Nodes, showing the old node layout.
- VAE Decode Batched 🎥🅥🅗🅢: some widgets didn’t properly migrate to Vue Nodes, showing the old node layout.
- Split Latents 🎥🅥🅗🅢: some widgets didn’t properly migrate to Vue Nodes, showing the old node layout.
- Split Masks 🎥🅥🅗🅢: some widgets didn’t properly migrate to Vue Nodes, showing the old node layout.
- Repeat Latents 🎥🅥🅗🅢: some widgets didn’t properly migrate to Vue Nodes, showing the old node layout.
- Repeat Masks 🎥🅥🅗🅢: some widgets didn’t properly migrate to Vue Nodes, showing the old node layout.
- Select Every Nth Latent 🎥🅥🅗🅢: some widgets didn’t properly migrate to Vue Nodes, showing the old node layout.
- Select Every Nth Mask 🎥🅥🅗🅢: some widgets didn’t properly migrate to Vue Nodes, showing the old node layout.
- Select Latest 🎥🅥🅗🅢: some widgets didn’t properly migrate to Vue Nodes, showing the old node layout.

## comfyui-florence2
- Florence2ModelLoader: dropdown shows no model options.

## comfyui-animatediff-evolved
- Load CameraCtrl Poses (File) 🎭🅐🅓②: dropdown shows no options.
- Load CameraCtrl Poses (Path) 🎭🅐🅓②: cannot retrieve file path.

## comfyui-impact-pack
- ONNXDetectorProvider: dropdown shows no model options.

## comfyui-impact-pack
- SEGSOrderedFilterDetailerHookProvider: ascending/descending widget value for 'order' is hidden in Vue node (shows correctly in legacy node layout)
- SEGSRangeFilterDetailerHookProvider: inside/outside widget value for 'mode' is hidden in Vue node (shows correctly in legacy node layout)
- Switch (latent/legacy): select_on_execution/select_on_prompt widget value for 'set_mode' is hidden in Vue node (shows correctly in legacy node layout)
- Switch (SEGS/legacy): select_on_execution/select_on_prompt widget value for 'set_mode' is hidden in Vue node (shows correctly in legacy node layout)
- Switch (Any): select_on_execution/select_on_prompt widget value for 'set_mode' is hidden in Vue node (shows correctly in legacy node layout)
- Inversed Switch (Any): select_on_execution/select_on_prompt widget value for 'set_mode' is hidden in Vue node (shows correctly in legacy node layout)
- SEGSDetailer For Video (SEGS/pipe): bbox/crop_region widget value for 'guide_size_for' is hidden in Vue node (shows correctly in legacy node layout)
- SEGS Filter (range): inside/outside widget value for 'mode' is hidden in Vue node (shows correctly in legacy node layout)
- SEGS Filter (ordered): inside/outside widget value for 'mode' is hidden in Vue node (shows correctly in legacy node layout)
- ImpactConditionalBranch: true/false widget value for 'cond' is hidden in Vue node (shows correctly in legacy node layout)
- ImpactConditionalBranchSelMode: true/false widget value for 'cond' is hidden in Vue node (shows correctly in legacy node layout)
- ImpactBoolean: true/false widget value for 'value' is hidden in Vue node (shows correctly in legacy node layout)
- ImpactMinMax: max/min widget value for 'mode' is hidden in Vue node (shows correctly in legacy node layout)
- Control Bridge: active/(stop/mute/bypass) widget value for 'mode' is hidden in Vue node (shows correctly in legacy node layout)
- Remote Boolean (on prompt): true/false widget value for 'value' is hidden in Vue node (shows correctly in legacy node layout)

## comfyui-kjnodes
- BOOL Constant: true/false widget value for 'value' is hidden in Vue node (shows correctly in legacy node layout)
- Conditioning Multi Combine: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to legacy node layout --> back to Vue node)
- Mask Batch Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to legacy node layout --> back to Vue node)
- Cross Fade Images Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to legacy node layout --> back to Vue node)
Image Add Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to legacy node layout --> back to Vue node)
Image Batch Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to legacy node layout --> back to Vue node)
- Image Concatenate Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to legacy node layout --> back to Vue node)
- Transition Images Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to legacy node layout --> back to Vue node)
- Join String Multi: 'update inputs' do not update inputs of node after 'inputcount' is changed (only works if switch to legacy node layout --> back to Vue node)
- Empty Latent Image Custom Presets: no presets available from drop-down, seems unintuitive.
- Lazy Switch KJ: true/false widget value for 'switch' is hidden in Vue node (shows correctly in legacy node layout)
- Spline Editor: unable to interact with the node UI using Vue node (works correctly in legacy node layout)
- Points Editor: unable to interact with the node UI using Vue node (works correctly in legacy node layout)
- GGUFLoaderKJ: dropdown shows no model options.

## comfy_extras
- HypernetworkLoader: dropdown shows no model options.
- PhotoMakerLoader: dropdown shows no model options.
- Webcam Capture: 'unable to load webcam, please ensure access is granted. requested device not found' error on node.
- Record Audio: difficult to use, unable to stop/start recording effectively + recording audio ui on node extends out of the node itself for Vue node.
- Load 3D & Animation: uploaded file appears with random numbers/letters for filename on node.
- Boolean: true/false widget value for 'value' is hidden in Vue node (shows correctly in legacy node layout)
- Preview as Text: markdown/plaintext widget value for 'previewmode' is hidden in Vue node (shows correctly in legacy node layout)

## misc
- collapsed nodes extend in size when switchign to legacy node layout --> back to Vue node 
- collapsed nodes seem to have the correct width for the node (so it shows the node name + custom node name), could be used as a base to resize all nodes correctly once loaded in?
- unable to set seed to a 'fixed' option with the options available in the new seed layout for Vue node.