<template>
    <v-data-table-virtual
        :headers="headers"
        :items="dataList"
        height="100dvh"
        item-value="name"
    >
        <template v-slot:item.name="item">
            <v-btn @click="updateData(item.item)" width="80%">{{ item.item.name }}</v-btn>
        </template>
        <template v-slot:item.edit="{item}">
            <v-icon @click="deleteItem(item)">mdi-pencil</v-icon>
        </template>
        <template v-slot:item.del="{item}">
            <DeleteItemPopup @delete="() => deleteItem(item)"/>
        </template>
    </v-data-table-virtual>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { ref, reactive, computed} from "vue";
import { VDataTableVirtual, VBtn } from "vuetify/components";
import DeleteItemPopup from "./DeleteItemPopup.vue";

    const emit = defineEmits(["nextItem", "delete"])
    defineProps({
            dataList: Object
    })

    function deleteItem(item){
        emit("delete", item)
    }
    function editItem(item){
        return 0
    }
    function updateData(item){
        emit("nextItem", item)
    }

    const headers = ref([
            { title: 'Name', align: 'start', key: 'name' },
            { title: 'Priority', align: 'end', key: 'priority' },
            { title: 'Est Time', align: 'end', key: 'est_time' },
            { title: 'Resource', align: 'end', key: 'resource' },
            { title: 'Available', align: 'end', key: 'availability' },
            { title: 'Completed', align: 'end', key:'completed'},
            { title: 'Edit', align: 'end', key: 'edit' },
            { title: "Del", align: "end", key: "del"},
    ])

</script>

