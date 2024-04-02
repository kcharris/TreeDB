<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
import CreateNewItemPopup from "./components/CreateNewItemPopup.vue";
import FullDetails from "./components/FullDetails.vue"
import LeftNavBar from "./components/LeftNavBar.vue";
import MainList from "./components/MainList.vue";
import {ref} from "vue"

    const values = ref({
      v: {}
    })
    function addItem(itemObject:object){
      values.value.v = itemObject;
      // send to rust.

    }
</script>

<template>
  <v-app>
  <v-app-bar title="Application bar"></v-app-bar>
  <v-main>
    <LeftNavBar/>
    <FullDetails/>
    <v-toolbar>
      <v-spacer/>
      <CreateNewItemPopup @send-values="addItem"/>
        <!-- <span>{{ values }}</span> -->
    </v-toolbar>
    <MainList />
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
