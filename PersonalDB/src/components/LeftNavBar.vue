<script setup lang="ts">

    const emit = defineEmits(["updatePage"])
    function changePage(pageNum:number){
        emit("updatePage", pageNum)
    }
</script>
<template>
    <v-navigation-drawer expand-on-hover rail location="left">
        <v-list-item prepend-icon="mdi-list-box" @click="changePage(0)" title="List"></v-list-item>
        <v-divider></v-divider>
        <v-list-item prepend-icon="mdi-tag" @click="changePage(1)" title="Tags"></v-list-item>
        <v-divider></v-divider>
        <v-list-item prepend-icon="mdi-database" @click="changePage(2)" title="Database Manager"></v-list-item>
        <v-divider></v-divider>
        <v-list-item prepend-icon="mdi-backup-restore" @click="changePage(3)" title="Backup Manager"></v-list-item>
        <v-divider></v-divider>
        <v-list-item prepend-icon="mdi-information-variant-box" @click="changePage(4)" title="Help and Info"></v-list-item>
    </v-navigation-drawer>
</template>