<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
import CreateNewItemPopup from "./components/CreateNewItemPopup.vue";
import FullDetails from "./components/FullDetails.vue"
import LeftNavBar from "./components/LeftNavBar.vue";
import MainList from "./components/MainList.vue";
import {ref} from "vue"
import { invoke } from "@tauri-apps/api/tauri";


    const currParent = ref(NaN)
    // const values = ref({
    //   v: {}
    // })
    async function addItem(itemObject: any){
      itemObject.parent = currParent.value
      let strObject = JSON.stringify(itemObject)
      // currParent.value = 9
      await invoke("add_item", {payload: strObject})
    }
</script>

<template>
  <v-app>
  <v-app-bar title="Application bar"></v-app-bar>
  <v-main>
    <LeftNavBar/>
    <FullDetails/>
    <v-toolbar>
      <v-label>{{ currParent }}</v-label>
      <v-spacer/>
      <CreateNewItemPopup @send-values="addItem"/>
        <!-- <span>{{ values }}</span> -->
    </v-toolbar>
    <MainList />h
    </v-main>
  </v-app>
  
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
