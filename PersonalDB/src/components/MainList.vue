<script setup lang="ts">
import { ref} from "vue";
import { VDataTableVirtual, VBtn} from "vuetify/components";
import DeleteItemPopup from "./DeleteItemPopup.vue";

    const emit = defineEmits(["nextItem", "delete", "edit"])
    defineProps<{
            dataList: any
    }>()

    function getName(item: any): string {
        if (typeof item.item === "object"){
            return item.item.name
        }
        else{
            return "Error"
        }
    }
    
    function editItem(item: any){
        emit("edit", item)
    }

    function deleteItem(item: any){
        emit("delete", item)
    }
    
    function updateData(item: any){
        emit("nextItem", item)
    }

    const headers: any = ref([
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

<template>
    <v-data-table-virtual
        :headers = "headers"
        :items ="dataList"
        density="compact"
        class="fill-height"
        height="60dvh"
        fixed-header
        multi-sort
    >
        <template v-slot:item.name="item">
            <v-btn max-width="350" density="comfortable" @click="updateData(item.item)" class="w-100 text-none text-truncate">{{ getName(item) }}</v-btn>
        </template>
        <template v-slot:item.edit="{item}">
            <v-icon @click="editItem(item)">mdi-pencil</v-icon>
        </template>
        <template v-slot:item.del="{item}">
            <DeleteItemPopup @delete="() => deleteItem(item)"/>
        </template>
    </v-data-table-virtual>
</template>
