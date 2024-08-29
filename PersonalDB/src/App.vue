<script setup lang="ts">

import LeftNavBar from "./components/LeftNavBar.vue";
import ListPage from "./components/list-page/ListPage.vue"
import SettingsPage from "./components/settings-page/SettingsPage.vue"
import DBManagerPage from "./components/db-manager-page/DBManagerPage.vue"
import BackupManagerPage from "./components/backup-manager-page/BackupManagerPage.vue"
import {ref, onBeforeMount} from "vue"
import { invoke } from "@tauri-apps/api/tauri";
  onBeforeMount(async () => {
    // set db_name to the name in the text file.
    db_name.value = await invoke("get_db_name")
    page.value = 0
  })

  const page = ref(-1)
  const db_name = ref()
  const path = ref("HOME:/")
  
  function setPath(p: string){
    path.value = "HOME:/" + p
  }
  function updatePage(pageNum: number){
    if(page.value != pageNum){
      page.value = pageNum
      setPath("")
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
      <ListPage v-model="db_name" @send-path="setPath"/>
    </template>
    <template v-if="page == 1">
      <DBManagerPage v-model="db_name"/>
    </template>
    <template v-if="page == 2">
      <BackupManagerPage/>
    </template>
    <template v-if="page == 3">
      <SettingsPage/>
    </template>
    </v-main>
  </v-app>
  
</template>
