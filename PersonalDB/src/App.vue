<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
import CreateNewItemPopup from "./components/CreateNewItemPopup.vue";
import FullDetails from "./components/FullDetails.vue"
import LeftNavBar from "./components/LeftNavBar.vue";
import MainList from "./components/MainList.vue";
import {ref, computed} from "vue"
import { invoke } from "@tauri-apps/api/tauri";


    const curr_parent = ref({
      name: "",
      id: NaN,
      parent: NaN,
      priority: NaN,
      est_time: NaN,
      resource: "",
      start_date: "",
      end_date: "",
      availability: "",
      completed: false,
      description: "",
    })
    const test = ref("Starter val")
    const data_str = ref("")
    const data_list = computed(() => {return data_str.value == "" ? [] : JSON.parse(data_str.value)})

    async function addItem(itemObject: any){
      itemObject.parent = curr_parent.value
      let strObject = JSON.stringify(itemObject)
      // currParent.value = 9
      await invoke("add_item", {payload: strObject})
    }
    async function getList(){
      data_str.value = await invoke("find_items_by_parent_id", {id: curr_parent.value.id})
    }
    async function nextItem(new_parent: any){
      curr_parent.value = new_parent
      getList()
    }
</script>

<template>
  <v-app>
  <v-app-bar title="Application bar"></v-app-bar>
  <v-main>
    <LeftNavBar/>
    <FullDetails/>
    <v-toolbar>
      <v-label>{{ curr_parent }} </v-label>
      <v-label>{{ test }}</v-label>
      <v-btn :onclick="getList">"Refresh"</v-btn>
      <v-spacer/>
      <CreateNewItemPopup @send-values="addItem"/>
        <!-- <span>{{ values }}</span> -->
    </v-toolbar>
    <MainList :data-list = "data_list"  @next-item="nextItem"/>
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
