<script setup lang="ts">

import LeftNavBar from "./components/LeftNavBar.vue";
import ListPage from "./components/list-page/ListPage.vue"
// import SettingsPage from "./components/settings-page/SettingsPage.vue"
import DBManagerPage from "./components/db-manager-page/DBManagerPage.vue"
import BackupManagerPage from "./components/backup-manager-page/BackupManagerPage.vue"
import TagPage from "./components/tag-page/TagPage.vue"
import HelpPage from "./components/help-page/HelpPage.vue"
import {ref, onBeforeMount, watch} from "vue"
import { invoke } from "@tauri-apps/api/tauri";
import {Item, Tag } from "./item-types.ts"

  onBeforeMount(async () => {
    // set db_name to the name in the text file.
    db_name.value = await invoke("get_db_name")
    let tags_str:string = await invoke("get_tags", {dbName: db_name.value})
    tags.value = tags_str == "" ? [] : JSON.parse(tags_str)
    page.value = 0
  })

  const page = ref(-1)
  const db_name = ref()
  const tags = ref<Tag[]>([])
  const path = ref("HOME:/")
  const path_stack = ref<Item[]>([])

  watch(db_name, async () => {
    path_stack.value = []
    path.value = "HOME:/"
    let tags_str:string = await invoke("get_tags", {dbName: db_name.value})
    tags.value = tags_str == "" ? [] : JSON.parse(tags_str)
  })
  
  function setPath(p: string){
    path.value = "HOME:/" + p
  }
  
  function updatePage(pageNum: number){
    if(page.value != pageNum){
      page.value = pageNum
    }
  }
</script>

<template>
  <v-app>
  <v-app-bar color="blue-grey-lighten-5" density="compact" :elevation="2">
    <p class="ml-5"><div class="text-truncate text-nowrap">{{ path }}</div></p>
    <v-spacer/>
  </v-app-bar>
  <LeftNavBar @update-page="updatePage"/>

  <v-main fluid class="d-flex flex-column">
    <template v-if="page == 0">
      <ListPage v-model:db-name="db_name" v-model:path-stack="path_stack" :tags="tags" @send-path="setPath"/>
    </template>
    <template v-if="page == 1">
      <TagPage v-model:db-name="db_name" v-model:tags="tags"/>
    </template>
    <template v-if="page == 2">
      <DBManagerPage v-model="db_name"/>
    </template>
    <template v-if="page == 3">
      <BackupManagerPage/>
    </template>
    <template v-if="page == 4">
      <HelpPage/>
    </template>
    </v-main>
  </v-app>
  
</template>
