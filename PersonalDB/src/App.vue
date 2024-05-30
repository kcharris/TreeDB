<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
import CreateNewItemPopup from "./components/CreateNewItemPopup.vue";
import FullDetails from "./components/FullDetails.vue"
// import LeftNavBar from "./components/LeftNavBar.vue";
import MainList from "./components/MainList.vue";
import EditItemPopup from "./components/EditItemPopup.vue"
import {ref, computed} from "vue"
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted } from "vue";
    onMounted(()=> {
      getList()
    })
    const path_stack = ref(Array())
    const path = computed(() => {
      let res_str = ""
      if (path_stack.value.length > 0 ){
        let arr_map: string[] = path_stack.value.map((s) => s.name.replaceAll(" ", "-"))
        res_str = arr_map.join("/")
      }
      return "HOME:/" + res_str
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
      description: "This is an example of the fully available items details. This Cleaning item serves as a categorical item that will contain other items.",
    }
    const curr_parent = ref(default_item)
    const data_str = ref("")
    const name_filter = ref("")
    const data_list = computed(() => {
      let res = data_str.value == "" ? [] : JSON.parse(data_str.value)
      if (name_filter.value != ""){
        return res.filter((obj:any) => containsSubsequence(obj.name.toLowerCase(), name_filter.value))
      }
      return res
    })
    const item_to_edit = ref({})
    const edit_dialog_bool = ref(false)

    function containsSubsequence(s:string, sub:string){
      if (s.length < sub.length){
        return false
      }
      let i = 0
      let j = 0
      while (i < s.length && j < sub.length){
        if (s[i] == sub[j]){
          j += 1
        }
        i += 1
      }
      if (j == sub.length){
        return true
      }
      return false
    }
    async function addItem(item_object: any){
      item_object.parent = curr_parent.value.id
      let str_object = JSON.stringify(item_object)
      // currParent.value = 9
      await invoke("add_item", {payload: str_object})
      getList()
    }
    async function deleteItem(item_object:any){
      await invoke("delete_item", {id: item_object.id})
      getList()
    }
    async function updateItem(item_object: any){
      let str_object = JSON.stringify(item_object)
      await invoke("update_item", {payload: str_object})
      getList()
    }

    function getEditItemPopup(item_object: any){
      item_to_edit.value = item_object
      edit_dialog_bool.value = true
    }

    async function getList(){
      name_filter.value = ""
      data_str.value = await invoke("find_items_by_parent_id", {id: curr_parent.value.id})
    }
    async function nextItem(item_object: any){
      path_stack.value.push(JSON.parse(JSON.stringify(item_object)))
      curr_parent.value = item_object
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
  <v-app-bar density="compact">
    <v-label class="ml-5"><div class="text-truncate text-nowrap">{{ path }}</div></v-label>
    <v-spacer/>
  </v-app-bar>
  <EditItemPopup v-model = "edit_dialog_bool" :item_to_edit = "item_to_edit" @send-values="updateItem"/>

  <v-main>
    <!-- <LeftNavBar/> -->
    <FullDetails :parent = "curr_parent"/>
    <v-toolbar density="compact">
      <v-btn class="bg-primary mr-2 ml-5 my-auto" :onclick="navHome">Home</v-btn>
      <v-btn class="bg-primary mr-10 my-auto" :onclick="navBack" variant="text">Back</v-btn >
      <v-text-field
        class="mt-5"
        density="comfortable"
        v-model="name_filter"
        max-width="275"
        maxlength="40"
        placeholder="Search"
        outlined
        single-line
        clearable
      ></v-text-field>
      <v-spacer/>
      <CreateNewItemPopup class = "bg-primary" @send-values="addItem"/>
    </v-toolbar>
    <MainList :data-list = "data_list" @edit="getEditItemPopup"  @next-item="nextItem" @delete="deleteItem"/>
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
