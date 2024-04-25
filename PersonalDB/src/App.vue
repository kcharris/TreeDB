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

    const path_stack = ref(Array())
    const path = computed(() => {
      let res_str = ""
      if (path_stack.value.length > 0 ){
        let arr_map: string[] = path_stack.value.map((s) => s.name.replace(" ", "-"))
        res_str = arr_map.join("/")
      }
      return "HOME:/" + res_str
    })
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
    const default_item = {
      name: "default",
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
    }
    const data_str = ref("")
    const data_list = computed(() => {return data_str.value == "" ? [] : JSON.parse(data_str.value)})

    async function addItem(itemObject: any){
      itemObject.parent = curr_parent.value.id
      let strObject = JSON.stringify(itemObject)
      // currParent.value = 9
      await invoke("add_item", {payload: strObject})
    }
    async function getList(){
      data_str.value = await invoke("find_items_by_parent_id", {id: curr_parent.value.id})
    }
    async function nextItem(new_parent: any){
      path_stack.value.push(JSON.parse(JSON.stringify(new_parent)))
      curr_parent.value = new_parent
      getList()
    }
    async function navBack(){
        if (path_stack.value.length > 0){
          path_stack.value.pop()
          if (path_stack.value.length > 0){
            curr_parent.value = path_stack.value[path_stack.value.length - 1]
          }
          else{
            curr_parent.value = default_item
          }
          getList()
        }
    }
    async function navHome(){
      path_stack.value = []
      curr_parent.value = default_item
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
      <v-btn class="bg-primary mr-2" :onclick="navHome">Home</v-btn>
      <v-btn class="bg-primary mr-10" :onclick="navBack" variant="text">Back</v-btn >
      <v-label>{{ path }}</v-label>
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
