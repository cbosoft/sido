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
        freq: { reserved: true, name: "freq", kind: "Constant", value: "-" },
        osc1: { kind: "Sine" },
        mult: { kind: "MultChannels" },
        ctl: { reserved: true, name: "ctl", kind: "Constant", value: "-" },
        out: { reserved: true, name: 'out', kind: "Sink" }
      },
      edges: {
        edge_freq_osc1: { source: 'freq', target: 'osc1' },
        edge_osc1_mult: { source: 'osc1', target: 'mult' },
        edge_ctl_mux: { source: 'ctl', target: 'mult' },
        edge_mux_out: { source: 'mult', target: 'out' },
      }
    },
    selected_nodes: [],
    selected_edges: [],
  });

  function update_patch() {
    // serialisable patch network
    const patch = {
      nodes: {},
      edges: [],
    };
    const node_connections = {}; // to track channels
    for (const [k, v] of Object.entries(data.patch.nodes)) {
      if (v.reserved) continue;
      patch.nodes[k] = { op: v.kind };
      node_connections[k] = 0;
    }
    for (const [_, edge] of Object.entries(data.patch.edges)) {
      const source = edge.source;
      const t_channel = node_connections[edge.target] ?? 0;
      const target = `${edge.target}:${t_channel}`;
      patch.edges.push([source, target]);
      if (node_connections[edge.target] !== undefined) {
        node_connections[edge.target] += 1;
      }
    }
    set_patch(patch);
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
