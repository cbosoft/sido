<script setup>
  import { ref, reactive, computed } from "vue";
  import * as vNG from "v-network-graph";
  import { invoke } from "@tauri-apps/api/core";

  const { data } = defineProps(['data']);

  const initial_layout = {
    nodes: {
      freq: { x: 0, y: 0 },
      ctl: { x: 0, y: 100 },
      out: { x: 200, y: 100 },
      mult: { x: 100, y: 100 },
      osc1: { x: 100, y: 0 },
    }
  };

  function delete_node(node_id) {
    const node = data.patch.nodes[node_id];
    if (!node.reserved) {
      delete data.patch.nodes[node_id];
    }
  }

  function delete_edge(edge_id) {
    delete data.patch.edges[edge_id];
  }

  const eventHandlers = {
    "node:click": ({ node, event }) => {
      if (event.ctrlKey) {
        delete_node(node);
      }
      else if (event.shiftKey) {
        // select multiple
        data.selected_nodes.push(node);
        if (data.selected_nodes.length > 2) {
          // shouldn't really happen...
          data.selected_nodes = data.selected_nodes.slice(1);
        }
        else if (data.selected_nodes.length == 2) {
          const [source, target] = data.selected_nodes;
          data.selected_nodes = [];
          join(source, target);
        }
      }
      else {
        data.selected_nodes = [node];
      }
    },
    "edge:click": ({ edge, event }) => {
      delete_edge(edge);
    },
    "view:dblclick": ({ event }) => {
      addNode();
    }
  }

  const nextNodeIndex = ref(Object.keys(data.patch.nodes).length + 1)
  const nextEdgeIndex = ref(Object.keys(data.patch.edges).length + 1)
  
  function addNode() {
    const nodeId = `node${nextNodeIndex.value}`;
    const name = `N${nextNodeIndex.value}`;
    data.patch.nodes[nodeId] = { name, kind: "WhiteNoise" };
    nextNodeIndex.value++;
  }
  
  const reservedNodes = ['freq', 'ctl', 'out'];
  
  function join(source, target) {
    if (source === target) return;
    const edgeId = `edge_${source}_${target}`;
    data.patch.edges[edgeId] = { source, target };
    nextEdgeIndex.value++;
  }

  const g_spacing = 50;
  const g_w = 1;
  const grid_line = {
    color: "#dddddd11",
    width: g_w,
    dasharray: 0, //[g_w, g_spacing - 2*g_w, 0, g_w],
  };

  const configs = vNG.defineConfigs({
    node: {
      //selectable: 2,
      normal: {
        radius: 20,
        color: "#000000ff",
        strokeColor: "#ddd",
        strokeWidth: node => node.reserved ? 4 : 2,
      },
      hover: {
        radius: 22,
        color: "transparent",
        strokeColor: "#fff",
        strokeWidth: node => node.reserved ? 4 : 2,
      },
      label: {
        visible: node => node.reserved,
        fontSize: 12,
        fontFamily: "monospace",
        color: "#ddd",
        direction: "center",
      },
      focusring: {
        color: "#ddd",
        width: 1,
      },
    },
    edge: {
      margin: 5,
      normal: {
        width: 3,
        color: "#ddd",
      },
      // selected: { color: "#fff", dasharray: 2, },
      hover: { color: "#ddd", dasharray: 4, },
      marker: {
        target: {
          type: "circle",
          width: 2,
          height: 2,
        }
      }
    },
    view: {
      layoutHandler: new vNG.GridLayout({ grid: 25 }),
      doubleClickZoomEnabled: false,
      grid: {
        visible: true,
        interval: 50,
        thickInterval: 100,
        line: grid_line,
        thick: grid_line,
      }
    }
  })
</script>

<template>
  <div class="patch-network-editor">
    <v-network-graph
      class="patch-network"
      v-model:selected-nodes="data.selected_nodes"
      v-model:selected-edges="data.selected_edges"
      :nodes="data.patch.nodes"
      :edges="data.patch.edges"
      :configs="configs"
      :layouts="initial_layout"
      :event-handlers="eventHandlers"
    >
      <template #override-node="{ nodeId, scale, config, ...slotProps }">
        <circle
          :r="config.radius * scale"
          fill="#00000011"
          v-bind="slotProps"
        />
        <image
          :x="-config.radius * scale"
          :y="-config.radius * scale"
          :width="config.radius * scale * 2"
          :height="config.radius * scale * 2"
          :xlink:href="`./modules/${data.patch.nodes[nodeId].kind}.svg`"
          v-if="!data.patch.nodes[nodeId].reserved"
        />
        <!-- circle for drawing stroke -->
        <circle
          :r="config.radius * scale"
          fill="none"
          stroke="#ddd"
          :stroke-width="2 * scale"
          v-bind="slotProps"
        />
      </template>
    </v-network-graph>
  </div>
</template>

<style scoped>
.patch-network-editor {
  position: relative;
  display: flex;
}

.patch-network {
  width: 100%;
  height: 100%;
}
</style>
