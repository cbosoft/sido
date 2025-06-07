<script setup>
  import { ref, reactive, computed } from "vue";
  import * as vNG from "v-network-graph";
  import { invoke } from "@tauri-apps/api/core";

  const { data } = defineProps(['data']);

  const initial_layout = {
    nodes: {
      freq: { x: 0, y: 0 },
      ctl: { x: 0, y: 100 },
      out: { x: 300, y: 100 },
    }
  };

  const eventHandlers = {
    "node:click": ({ node, event }) => {
      if (event.ctrlKey) {
        selectedNodes.value = [node];
        removeNode();
      }
      else {
        if (selectedNodes.value.length == 2) {
          selectedNodes.value = selectedNodes.value.slice(1);
        }
        selectedNodes.value.push(node);
        if (selectedNodes.value.length == 2) {
          join();
          selectedNodes.value = [];
        }
        data.selected_node = node;
      }
    },
    "edge:click": ({ edge, event }) => {
      delete data.patch.edges[edge];
    },
    "view:dblclick": ({ event }) => {
      addNode();
    }
  }

  const nextNodeIndex = ref(Object.keys(data.patch.nodes).length + 1)
  const nextEdgeIndex = ref(Object.keys(data.patch.edges).length + 1)
  
  const selectedNodes = ref([])
  const selectedEdges = ref([])
  
  function addNode() {
    const nodeId = `node${nextNodeIndex.value}`;
    const name = `N${nextNodeIndex.value}`;
    data.patch.nodes[nodeId] = { name };
    nextNodeIndex.value++;
  }
  
  const reservedNodes = ['freq', 'ctl', 'out'];
  function removeNode() {
    for (const nodeId of selectedNodes.value) {
      if (!reservedNodes.includes(nodeId)) {
        delete data.patch.nodes[nodeId];
      }
    }
  }
  
  function join() {
    if (selectedNodes.value.length !== 2) return;

    const [source, target] = selectedNodes.value;
    if (source === target) return;

    const edgeId = `edge${nextEdgeIndex.value}`;
    data.patch.edges[edgeId] = { source, target };
    nextEdgeIndex.value++;
  }

  const can_not_join = computed(() => (selectedNodes.value.length < 2));
  
  function removeEdge() {
    for (const edgeId of selectedEdges.value) {
      delete data.patch.edges[edgeId]
    }
  }

  function remove() {
    removeNode();
    removeEdge();
  }

  const configs = vNG.defineConfigs({
    node: {
      //selectable: 2,
      normal: {
        radius: 20,
        color: "transparent",
        strokeColor: "#ddd",
        strokeWidth: node => reservedNodes.includes(node.name) ? 4 : 2,
      },
      hover: {
        radius: 22,
        color: "transparent",
        strokeColor: "#fff",
        strokeWidth: node => reservedNodes.includes(node.name) ? 4 : 2,
      },
      label: {
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
    },
    view: {
      layoutHandler: new vNG.GridLayout({ grid: 25 }),
      doubleClickZoomEnabled: false,
    }
  })
</script>

<template>
  <div class="patch-network-editor">
    <v-network-graph
      class="patch-network"
      v-model:selected-nodes="selectedNodes"
      v-model:selected-edges="selectedEdges"
      :nodes="data.patch.nodes"
      :edges="data.patch.edges"
      :configs="configs"
      :layouts="initial_layout"
      :event-handlers="eventHandlers"
    />
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
