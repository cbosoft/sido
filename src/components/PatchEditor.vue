<script setup>
  import { watch, reactive } from "vue";
  import * as vNG from "v-network-graph";
  import { invoke } from "@tauri-apps/api/core";

  async function set_patch(patch) {
    await invoke("set_patch", { patch });
  }

  import PatchNetworkEditor from "./PatchNetworkEditor.vue";
  import NodeEditor from "./NodeEditor.vue";

  const data = reactive({
    patch: {
      nodes: {
        freq: { name: 'freq', width: 5 },
        osc1: { name: 'osc1' },
        mux: { name: 'mux' },
        ctl: { name: 'ctl' },
        out: { name: 'out' }
      },
      edges: {
        e1: { source: 'freq', target: 'osc1' },
        e2: { source: 'osc1', target: 'mux' },
        e3: { source: 'ctl', target: 'mux' },
        e4: { source: 'mux', target: 'out' },
      }
    },
    selected_node: null
  });

  function update_patch() {
    // get serable repr of patch
    // send to backend
    console.log(data.patch);
  }

  watch(data.patch, update_patch);

</script>

<template>
  <div class="patch-editor">
    <div class="patch-list">
    </div>
    <PatchNetworkEditor :data="data" />
    <NodeEditor :data="data" />
  </div>
</template>

<style scoped>
.patch-editor {
  position: absolute;
  inset: 0;

  display: flex;
  flex-direction: row;
}
.patch-network-editor {
  flex-grow: 1;
}
.patch-list {
  border-right: 2px solid #ddd;
  width: 25%;
}
.node-editor {
  border-left: 2px solid #ddd;
  width: 25%;
}
</style>
