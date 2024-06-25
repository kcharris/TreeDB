<script setup lang="ts">
import { ref} from "vue";
import { VDataTableVirtual, VBtn} from "vuetify/components";
import DeleteItemPopup from "./DeleteItemPopup.vue";
import { invoke } from "@tauri-apps/api/tauri";


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
            { title: 'EstTime', align: 'end', key: 'est_time' },
            { title: 'Resource', align: 'end', key: 'resource' },
            { title: 'StartDate', align: 'end', key: 'start_date' },
            { title: 'Available', align: 'end', key: 'availability' },
            { title: 'Completed', align: 'end', key:'completed'},
            { title: 'Edit', align: 'end', key: 'edit' },
            { title: "Del", align: "end", key: "del"},
    ])
    function openDir(link: string){
        invoke("open_file_explorer", {dirAddress: link})
    }

</script>

<template>
    <v-data-table-virtual
        :headers = "headers"
        :items ="dataList"
        density="compact"
        class="fill-height overflow-x-auto"
        height="60dvh"
        fixed-header
        multi-sort
    >
    
        <template v-slot:item.resource="item">
            <v-btn class="text-none" 
                v-if="item.item.resource_type == undefined && item.item.resource_link != undefined"
                density="compact"
                variant="text"
                >
                no target selected
            </v-btn>
            <v-btn class="text-none"
                v-else-if="item.item.resource != undefined && item.item.resource_link == undefined"
                density="compact"
                variant="text"
                >
                {{item.item.resource}}
            </v-btn>
            <v-btn
                class="text-none text-primary text-decoration-underline" 
                density="compact"
                v-else-if="item.item.resource_type=='web'"
                :href="item.item.resource_link"
                target="_blank"
                variant="text"
                >
                {{ item.item.resource == undefined ? "link" : item.item.resource}}
            </v-btn>
            <v-btn
                class= "text-none text-primary text-decoration-underline"
                density="compact"
                v-else-if="item.item.resource_type=='dir'"
                @click="openDir(item.item.resource_link)"
                variant="text"
                >
                {{ !item.item.resource == undefined ? "link" : item.item.resource}}
            </v-btn>
        </template>
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
