<script setup lang="ts">

import {Item, Tag} from "../../item-types";
import CreateAndEditPopup from "./CreateAndEditPopup.vue";
import FullDetails from "./FullDetails.vue"
import MainList from "./MainList.vue";
import FullDetailsHome from "./FullDetailsHome.vue"
import {ref, computed, watch, onMounted} from "vue"
import { invoke } from "@tauri-apps/api/tauri";

    onMounted(async ()=> {
      curr_parent.value = path_stack.value.length == 0 ? default_item : path_stack.value[path_stack.value.length-1]
      await getList()
    })
    const emits = defineEmits(["sendPath"])
    const props = defineProps(["tags"])
    const db_name = defineModel("dbName", {required:true})
    const path_stack = defineModel<Item[]>("pathStack", {required: true})
    watch(path_stack, (ps)=> {
        let res_str = ""
        if (path_stack.value.length > 0 ){
            let arr_map: string[] = ps.map((s) => s.name.replaceAll(" ", "-"))
            res_str = arr_map.join("/")
        }
      let res = res_str
        emits("sendPath", res)
    }, {deep: true})
    const default_item: Item = {
      name: "default",
      id: undefined
    }
    const curr_parent = ref<Item>(default_item)
    const data_str = ref("")
    const name_filter = ref("")
    const data_list = computed<Item[]>(() => {
      let res = data_str.value == "" ? [] : JSON.parse(data_str.value)
      if (name_filter.value){
        res = res.filter((obj:any) => containsSubsequence(obj.name.toLowerCase(), name_filter.value.toLowerCase()))
      }
      if (tags_selected.value.length > 0){
        let item_tags: Set<Number> = new Set()
        let tag_set:Set<string> = new Set(tags_selected.value)
        props.tags.forEach((t:Tag)=>{
          if (tag_set.has(t.name)){
            item_tags.add(Number(t.id))
          }
        })
        res = res.filter((obj:Item) => {return (new Set(item_tag_map.value?.get(obj.name))).isSupersetOf(item_tags) == true})
      }
      return res
    })
    const item_to_edit = ref<Item>(default_item)
    const edit_dialog_bool = ref(false)
    const can_edit = computed(() => !(curr_parent.value.id))
    const tags_selected = ref<string[]>([])
    const tag_names = computed<string[]>(()=> props.tags.map((t: Tag)=>{return t.name}))
    const item_tag_map = ref<Map<string, Set<Number>>>()
    const tags_owned = ref<Set<Number>>(new Set())
    const is_loading = ref(false)

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

    async function addItem(payload:{item_object: Item, item_tags: Number[]}){
      let item_object = payload.item_object
      let item_tags = payload.item_tags
      if (curr_parent.value.id != undefined){
        item_object.parent_id = curr_parent.value.id
      }
      let str_object = JSON.stringify(item_object)
      item_object.id = await invoke("add_item", {dbName: db_name.value, payload: str_object})
      await updateItemTags(item_object, item_tags)
      await getList()
    }

    async function deleteItem(item_object:Item){
      await invoke("delete_item", {dbName: db_name.value, id: item_object.id})
      await getList()
    }

    async function updateItem(payload:{item_object:Item, item_tags:Number[]}){
      let item_object = payload.item_object
      let item_tags = payload.item_tags
      let str_object = JSON.stringify(item_object)
      await invoke("update_item", {dbName: db_name.value, payload: str_object})
      await updateItemTags(item_object, item_tags)
      if (item_object.id == curr_parent.value.id){
        str_object = await invoke("get_item_by_id", {dbName: db_name.value, id: item_object.id})
        curr_parent.value = JSON.parse(str_object)
      }
      else{
        await getList()
      }
    }
    async function updateItemTags(item: Item, item_tags: Number[]){
      // remove a tag if it exists in previous memory but not in item_tags
      let item_tag_set = new Set(item_tags)
      let previous_item_tag_set = new Set(item_tag_map.value?.get(item.name))
      let to_remove = previous_item_tag_set.difference(item_tag_set)
      // add a tag if it does not exist in previous memory and does exist in item_tags
      let to_add = item_tags.filter((t:Number) => {return !item_tag_map.value?.get(item.name)?.has(t)})

      to_remove?.forEach(async (id:Number)=>{
        await invoke('delete_item_tag', {dbName: db_name.value, itemId: item.id, tagId: id})
      })
      to_add?.forEach(async (id:Number)=>{
        await invoke('add_item_tag', {dbName: db_name.value, itemId: item.id, tagId: id})
      })
    }

    async function getTagsSelected(){
      let res:Set<Number> = new Set()
      if (item_to_edit.value.id != curr_parent.value.id){
        res = item_tag_map.value?.get(item_to_edit.value?.name as string) ?? new Set()
      }
      else{
        let res_str:string = await invoke("get_tags_by_item_id", {dbName: db_name.value, id: item_to_edit.value.id})
        let res_tags = JSON.parse(res_str)
        res = res_tags.map((t:Tag)=> {return t.id})
      }
      return res
    }

    async function getEditItemPopup(item_object: Item){
      item_to_edit.value = item_object
      tags_owned.value = await getTagsSelected()
      edit_dialog_bool.value = true
    }

    async function editCurrent(){
      await getEditItemPopup(curr_parent.value)
    }

    async function getList(){
      name_filter.value = ""
      is_loading.value = true
      data_str.value = await invoke("find_items_by_parent_id", {dbName: db_name.value, id: curr_parent.value.id})
      item_tag_map.value = await getItemTags()
      is_loading.value = false
    }

    async function getItemTags(){
      let data_map = new Map<string, Set<Number>>()
      data_list.value.forEach(async (item:Item) => {
        let tags_str:string = await invoke("get_tags_by_item_id", {dbName:db_name.value, id: item.id})
        let tags = tags_str == "" ? [] : JSON.parse(tags_str)
        let tag_set:Set<Number> = new Set(tags.map((t:Tag) => t.id))
        data_map.set(item.name, tag_set)
      })
      if (curr_parent.value.id != undefined){
        let tags_str:string = await invoke("get_tags_by_item_id", {dbName:db_name.value, id: curr_parent.value.id})
        let tags = tags_str == "" ? [] : JSON.parse(tags_str)
        let tag_set:Set<Number> = new Set(tags.map((t:Tag) => t.id))
        data_map.set(curr_parent.value.name, tag_set)
      }
      return data_map
    }

    async function nextItem(item_object: Item){
      path_stack.value.push(item_object)
      curr_parent.value = item_object
      await getList()
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
          await getList()
        }
    }
    async function navHome(){
      path_stack.value = []
      curr_parent.value = default_item
      await getList()
    }
    
</script>

<template>
    <CreateAndEditPopup
      v-model="edit_dialog_bool"
      :item_to_edit="item_to_edit"
      :tag_names="tag_names"
      :tags="props.tags"
      :tags_owned="tags_owned"
      @send-values="updateItem"
    />
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
          (+{{ tags_selected.length - 1}} others)
        </span>
      </template>
    </v-select>
    <v-spacer/>
    <v-btn :disabled="can_edit" @click="editCurrent" class="bg-primary mr-2">Edit</v-btn>
    <CreateAndEditPopup :tag_names="tag_names" :tags="props.tags" @send-values="addItem"/>
    </v-toolbar>
    <MainList :is-loading="is_loading" :data-list="data_list" @edit="getEditItemPopup"  @next-item="nextItem" @delete="deleteItem"/>
</template>