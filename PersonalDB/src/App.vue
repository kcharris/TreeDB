<script setup lang="ts">

import LeftNavBar from "./components/LeftNavBar.vue";
import ListPage from "./components/list-page/ListPage.vue"
import SettingsPage from "./components/settings-page/SettingsPage.vue"
import DBManagerPage from "./components/db-manager-page/DBManagerPage.vue"
import {ref, onMounted} from "vue"
  onMounted(() => {
    // set db_name to the name in the text file.
    db_name.value = "default" // placeholder
  })

  const page = ref(0)
  const db_name = ref()
  const path = ref("HOME:/")
  
  function setPath(p: string){
    path.value = p
  }
  function updatePage(pageNum: number){
    page.value = pageNum
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
      <DBManagerPage/>
    </template>
    <template v-if="page == 2">
      <SettingsPage/>
    </template>
    </v-main>
  </v-app>
  
</template>
