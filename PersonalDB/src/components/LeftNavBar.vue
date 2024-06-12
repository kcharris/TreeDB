<script setup>
    const emit = defineEmits(["updatePage"])

    function changePage(pageNum){
        emit("updatePage", pageNum)
    }
</script>
<template>
    <v-navigation-drawer permanent width="120" location="left">
        <v-list-item @click="changePage(0)" title="List"></v-list-item>
        <!-- <v-list-item link title="Tags"></v-list-item> -->
        <!-- <v-divider></v-divider> -->
        <!-- <v-list-item link title="DB Select"></v-list-item> -->
        <v-divider></v-divider>
        <v-list-item @click="changePage(1)" title="Settings"></v-list-item>
    </v-navigation-drawer>
</template>