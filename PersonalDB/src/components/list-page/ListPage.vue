<script setup lang="ts">

import {Item, Tag} from "../../item-types";
import CreateAndEditPopup from "./CreateAndEditPopup.vue";
import FullDetails from "./FullDetails.vue"
import MainList from "./MainList.vue";
import FullDetailsHome from "./FullDetailsHome.vue"
import {ref, computed, watch, onMounted} from "vue"
import { invoke } from "@tauri-apps/api/tauri";

    onMounted(()=> {
      getList()
    })
    const emits = defineEmits(["sendPath"])
    const props = defineProps(["tags"])
    const db_name = defineModel()
    const path_stack = ref(Array())
    watch(path_stack.value, (ps)=> {
        let res_str = ""
        if (path_stack.value.length > 0 ){
            let arr_map: string[] = ps.map((s) => s.name.replaceAll(" ", "-"))
            res_str = arr_map.join("/")
        }
      let res = res_str
        emits("sendPath", res)
    })
    const default_item: Item = {
      name: "default",
    }
    const curr_parent = ref(default_item)
    const data_str = ref("")
    const name_filter = ref("")
    const data_list = computed(() => {
      let res = data_str.value == "" ? [] : JSON.parse(data_str.value)
      if (name_filter.value){
        res = res.filter((obj:any) => containsSubsequence(obj.name.toLowerCase(), name_filter.value.toLowerCase()))
      }

      if (tags_selected.value.length > 0){
        let tag_name_set = new Set(tag_names.value)
        res = res.filter((obj:Item) => {item_tag_map.value?.get(obj.name)?.isSupersetOf(tag_name_set)})
      }

      return res
    })
    const item_to_edit = ref({})
    const edit_dialog_bool = ref(false)
    const can_edit = computed(() => !(curr_parent.value.id))
    const tags_selected = ref([])
    const tag_names = computed<string[]>(()=> props.tags.map((t: Tag)=>{return t.name}))
    const item_tag_map = ref<Map<string, Set<Number>>>()

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

    async function addItem(item_object: Item){
      item_object.parent_id = curr_parent.value.id
      let str_object = JSON.stringify(item_object)
      await invoke("add_item", {dbName: db_name.value, payload: str_object})
      getList()
    }

    async function deleteItem(item_object:Item){
      await invoke("delete_item", {dbName: db_name.value, id: item_object.id})
      getList()
    }

    async function updateItem(item_object: Item){
      let str_object = JSON.stringify(item_object)
      await invoke("update_item", {dbName: db_name.value, payload: str_object})
      if (item_object.id == curr_parent.value.id){
        str_object = await invoke("get_item_by_id", {dbName: db_name.value, id: item_object.id})
        curr_parent.value = JSON.parse(str_object)
      }
      else{
        getList()
      }
    }

    function getEditItemPopup(item_object: Item){
      item_to_edit.value = item_object
      edit_dialog_bool.value = true
    }
    function editCurrent(){
      getEditItemPopup(curr_parent.value)
    }

    async function getList(){
      name_filter.value = ""
      data_str.value = await invoke("find_items_by_parent_id", {dbName: db_name.value, id: curr_parent.value.id})
      item_tag_map.value = await getItemTags()
    }

    async function getItemTags(){
      let data_map = new Map<string, Set<Number>>()
      data_list.value.array.foreach(async (item:Item) => {
        let tags_str:string = await invoke("get_tag_by_item_id", {id: item.id})
        let tags = tags_str == "" ? [] : JSON.parse(tags_str)
        let tag_set:Set<Number> = new Set(tags.map((t:Tag) => t.id))
        data_map.set(item.name, tag_set)
      })
      return data_map
    }

    async function nextItem(item_object: Item){
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
    <CreateAndEditPopup v-model = "edit_dialog_bool" :item_to_edit = "item_to_edit" @send-values="updateItem"/>
    <template v-if="curr_parent.id != undefined">
    <FullDetails :parent = "curr_parent"/>
    </template>
    <template v-else>
    <FullDetailsHome />
    </template>

    <v-toolbar color="blue-grey-lighten-5" density="compact">
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
    <v-select max-width="250px" density="comfortable" v-model="tags_selected" :items="tag_names" class="ml-5 mt-5" label="Filter by Tag" multiple>
      <template v-slot:selection="{ item, index }">
        <v-chip v-if="index < 1">
          <span>{{ item.title}}</span>
        </v-chip>
        <span
          v-if="index===1"
          class="text-grey text-caption align-self-center"
        >
          (+{{ tags_selected.length - 2 }} others)
        </span>
      </template>
    </v-select>
    <v-spacer/>
    <v-btn :disabled="can_edit" @click="editCurrent" class="bg-primary mr-2">Edit</v-btn>
    <CreateAndEditPopup @send-values="addItem"/>
    </v-toolbar>
    <MainList :data-list="data_list" @edit="getEditItemPopup"  @next-item="nextItem" @delete="deleteItem"/>
</template>