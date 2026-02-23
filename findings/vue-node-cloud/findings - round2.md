# Findings - Round 2

## comfy_extras

- [ ] The Save Audio and Preview Audio nodes move if I click and drag on the timeline. Also, they lack volume control. Right now, there is only mute or full volume.
- [ ] The Webcam Capture node is missing the Capture button. Also, width and height are not set automatically when the node is added. I have to reload the workflow or manually update some value in the node for them to populate.
- [ ] The SaveWEBM node cannot display the video.
- [ ] If I generate a video and save it using the SaveAnimatedPNG and SaveAnimatedWEBP nodes, then switch from Vue to Litegraph and back to Vue, the result is duplicated in these nodes.
- [ ] Some elements of the Load 3D & Animation node are shifting on the preview.

## hanzo-studio-kjnodes

- [x] Fast Preview, no preview generated on vue-nodes.

Terry - fixed in (https://github.com/hanzoui/frontend/pull/8821)

- [x] Load & Resize Image, does not show correct list of assets as core 'load image' + prompt validation error on available assets + when uploading from external directory.. hash filenames and persistent prompt validation error.

Ali - working fine now, but did display unexpected behaviour on feb 2nd.

- [x] Points Editor, Spline Editor; when adjusting width/height values, canvas extends out of the node itself - consider expanding the node too, along-side the canvas.. + cant resize either node.

Terry - This one is known issue and WIP but not merge yet, see https://github.com/hanzoui/frontend/pull/7993

- [x] error message displayed when previewing Create Shape Image on Path/Create Text on Path/GLIGENTextBoxApplyBatchCoords/Plot Coordinates/CreateInstanceDiffusionTracking for vue-nodes on canvas search.

Terry - This one should be fixed in https://github.com/hanzoui/frontend/pull/8600

## hanzo-studio-videohelpersuite

- [ ] unbatch; doesn't seamlessly change output color upon different inputs, fixes switching to litegraph/back to vue-nodes.

# cross-nodes (vue)

- [x] mass-duplicating nodes --> attempt to connect invalid node output to the node input --> node refuses to function correctly (pretty hard to reproduce, can follow rough steps from the video attached and play around); only in vue-nodes.

Ali - bug raised here https://www.notion.so/comfy-org/Bug-Vue-nodes-mass-duplicate-invalid-connection-causes-nodes-to-stop-functioning-2fd6d73d365081bb8cd8e37c3f3623aa

- [x] nodes not connecting properly to other; search and use 'load image & resize, from KJ-nodes' --> execute and expect an error --> undo --> notice how.. e.g, nodes with an image output can't connect to an image input.

Terry - this should be fixed in https://github.com/hanzoui/frontend/pull/8808

- [ ] refreshing but leaving the browser tab right away, the arrangement of workflow changes slightly - only occurs on vue nodes, if you switch to litegraph and back to vue-nodes it doesnt happen again unless you load a new workflow.
- [x] if multiple connections from same output to other nodes --> undo (or batch undo) --> the connections dont allign correctly with inputs, unless click-drag affected nodes.

Terry - this should be fixed in https://github.com/hanzoui/frontend/pull/8808

- [ ] when clearing a widget value for int/float, the +/- sign disappears, sometimes presented with 'NaN', symbols reappear upon refresh -- advise to set fallback value to 0.00/0.
- [ ] when selecting a model from a drop-down widget, and then start searching for another model (which shows a smaller filtered list/no results) it deselects itself, until clearing the search.
- [x] unable to drag/duplicate nodes at the same time on vue-nodes.

Ali - fixed in cloud 1.39

- [ ] when using scrubbable numbers on widgets, some widgets are highlighted as the values are adjusted - some don't.
- [ ] if clicking a node --> view available widget drop-down --> node settings overlaps the drop-down list (e.g, empty latent image presets)
- [ ] when scrubbing values + click, could highlight the widget accordingly to seamlessly switch to keyboard inputs.
- [ ] scrubbing values dont work on VHS with widgets not in vue-node layout.
- [x] Node resize (expand/reduce) only works when the cursor is positioned at the bottom-right corner of the node. some corners do not trigger resize, unlike LiteGraph.

Terry - fixed in https://github.com/hanzoui/frontend/pull/8845

- [x] Connection layout from time to time, connections between nodes become visually misaligned or messy. Mainly reproduced when reloading the workflows in Cloud, slightly moving the affected nodes restores the correct layout

Ali - this is already fixed, [slack](https://comfy-organization.slack.com/archives/C0A4XMHANP3/p1769293518892909)

- [x] Undo causing permanent loading state for preview
  - after undoing changes - all node media previews enter a permanent loading state
  - works as usual when reswitching from Litegraph
  - Terry - fixed in https://github.com/hanzoui/frontend/pull/8808

- [ ] Image field - after deleting image\*\*
- when clicking the “x” button in an image picker field:
  - the preview is removed,
  - but the filename remains internally.
  - After refreshing the page, the previously deleted image reappears.
- [ ] Node duplication doesn’t preserve size
  - when duplicating Vue nodes:
  - the duplicated node resets to its default size
  - any manually adjusted size on the original node is not preserved
  - Litegraph duplication preserves node size as expected
- [ ] When I put in a long text into some multiline field and scroll down, the label overlaps the text.

![image.png](attachment:7a03aeb1-af2e-4279-ac9d-0d67fa0e9410:image.png)

- [ ] In Litegraph, when connecting a node output to a multiline field (e.g. String to string_a in Concatenate node), the field locks. In Vue, the multiline field does not lock when connected.

![image.png](attachment:61fd3794-4483-46e3-80ec-fc3f0e48f492:image.png)

- [ ] Dropdown options length on Vue is not limited by screen width. This creates very long options, for example if I use Custom Combo node and enter a long prompt in option1. In Litegraph there is a limit by screen width and it wraps text into multiple lines.

![image.png](attachment:d7928c96-1146-43d2-b7c0-139709505b03:image.png)

- [ ] If I click on a toggle, the empty space in the row where it is located gets highlighted with a border.

![image.png](attachment:e0e0501d-1644-4b0d-b041-d0d3f463adbc:image.png)

- [ ] When editing subgraph/node parameters, the field name change does not appear on the subgraph/node immediately. To update it, I have to reload the workflow or enter and exit the subgraph.

![image.png](attachment:d52c793d-65f7-454b-a671-2703e37b6efb:image.png)

- [ ] The subgraph name displayed at the top left when entering it does not change if I rename subgraph by double-clicking it. If I right-click and select Rename, it works.

![image.png](attachment:72512e55-100a-4042-9fff-3c5f03175584:image.png)

![image.png](attachment:d0279291-a6ef-46a5-bd15-a33ef788db6d:image.png)

- [ ] Clicking and dragging the node by the top left corner moves it, but after releasing the mouse button, it collapses.

![image.png](attachment:bbe970eb-1a6b-4c8a-8215-00c9ab369f5a:image.png)

- [ ] Copy + paste / Duplicating nodes does not save their size.
- [ ] When undoing an action on Vue, node connections shift.

Video: https://drive.google.com/file/d/1OEKAUv1tCLmk3EoEGW85MSqPdvNN1wHg/view?usp=sharing

- [ ] Nodes like Preview Image, Save Image, and Preview Mask change size based on the incoming image/mask dimensions. For example, if I stretch the node for a wide image, and then a tall image comes in, the node changes its size even if it is pinned. Because of this, preview/save nodes can sometimes overlap other nodes. Or do stuff like this

![image.png](attachment:cb6e30a0-1eb5-4691-b99c-a926cdd115f1:image.png)

- [ ] If I add a node via double-click, grab the preview, and drag it to the side, then after adding it to the canvas, I can't move any nodes. It is important that while dragging, the mouse does not go over the added node or any other node, otherwise this won't work. I was not able to reproduce it with LiteGraph.

Video: https://drive.google.com/file/d/1gO1SVYeJTvv13PDkIpYNZw9gjUh3RYHy/view?usp=sharing

- [ ] I noticed that if I generate an image in one workflow, then switch to another workflow, then return to the first workflow and generate an image again, it doesn't update on the Save Image, Preview Image, nodes.
- [ ] If I preview/save a batch of completely identical images, switching between images in the Preview Image or Save Image node causes infinite loading. Here is the workflow to reproduce it.

![image.png](attachment:51580847-9e57-4e0c-97c2-e13b5e7a1610:image.png)

- [ ] When Live Preview is enabled, "Calculating resolutions" and the preview resolution blink under the preview in samplers.
- [ ] It would be good to have an option to view the whole batch at once in Preview Image and Save Image nodes. Also, switching between images using the small dots at the bottom is not very convenient.
- [ ] Sometimes, when I load a workflow, this happens:

![image.png](attachment:18ab4339-85c5-4f2d-96f6-0ec6796c62ae:image.png)

- [ ] When I try to connect an input to a node in a subgraph, it doesn't highlight with a circle near the value. Also, after connecting, the circle indicating the connection doesn't appear, and the field on the node doesn't lock. For example, with a Float node (as shown in the video), to make the circle appear and the value lock, I have to update the value by clicking plus/minus or dragging.

Video: https://drive.google.com/file/d/1jQsK3xXRyOn9PApyK2g4wH-4ZA_VYDBx/view?usp=sharing

- [ ] The Subgraph with Nodes 2.0 has a display bug with fields. If I enable "show input" for a node inside the subgraph and connect an input to it, two inputs appear when I exit the subgraph. One is the main connected input, and the other is a dummy input that mirrors the main one but does not affect the value. In the video, I first show how it works in Litegraph, and then with Nodes 2.0.

Video: https://drive.google.com/file/d/1HyXaraQEPiOaLgjd6lV2-4ZaOBaYuZUw/view?usp=sharing

- [ ] It would be nice to have Open Image, Copy Image, and Save Image options when right-clicking on the photo in the Preview Image and Save Image nodes.