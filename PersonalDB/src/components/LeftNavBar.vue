<script setup lang="ts">
    const emit = defineEmits(["updatePage"])

    function changePage(pageNum:number){
        emit("updatePage", pageNum)
    }
</script>
<template>
    <v-navigation-drawer permanent width="135" location="left">
        <v-list-item @click="changePage(0)" title="List"></v-list-item>
        <!-- <v-list-item link title="Tags"></v-list-item> -->
        <v-divider></v-divider>
        <v-list-item @click="changePage(1)" title="DB Manager"></v-list-item>
        <v-divider></v-divider>
        <v-list-item @click="changePage(2)" title="Settings"></v-list-item>
    </v-navigation-drawer>
</template>