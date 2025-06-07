import { createApp } from "vue";
import VNetworkGraph from "v-network-graph";
import App from "./App.vue";

createApp(App)
  .use(VNetworkGraph)
  .mount("#app");
