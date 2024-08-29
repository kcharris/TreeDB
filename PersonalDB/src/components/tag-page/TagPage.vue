<script setup lang="ts">
    import { VTable, VBtn, VSheet} from "vuetify/components";
    import { ref, onMounted} from "vue";
    import { invoke } from "@tauri-apps/api/tauri";

    onMounted(()=>{
        refresh_tag_names();
    })

    const db_name = defineModel()
    const tag_names = ref<string[]>([])
    const success_dialog = ref(false)

    async function refresh_tag_names(){
        let filenames: string[] = await invoke("get_tag_names")
        tag_names.value = filenames
    }

    async function createTag(name: string){
        await invoke("add_tag", {dbName: db_name.value, name: name});
        refresh_tag_names();
    }

    function renameTag(payload: any){
        invoke("rename_tag", {dbName: db_name.value, payload: payload})
        refresh_tag_names();
    }

    async function deleteTag(id: Number){
        await invoke("delete_tag", {dbName: db_name.value, id: id })
        refresh_tag_names();
    }

</script>


<template>    
    <p>{{ db_name }}</p>
    <SuccessPopup v-model="success_dialog"></SuccessPopup>
    <v-toolbar color="blue-grey-lighten-5" density="compact">
    <v-spacer/>
    <CreatePopup @create="createTag"></CreatePopup>
    </v-toolbar>

    <v-sheet color="teal-lighten-2" class="fill-height mx-auto w-100">
        <!-- <v-card v-if="tag_names.length == 0" class="w-50 h-50 mx-auto mt-10">
            <v-card-title>No databases found</v-card-title>
        </v-card>
        <v-table v-else class="w-66 fill-height overflow-x-auto mx-auto">
            <thead>
                <tr>
                    <th >
                        Tag Name
                    </th>
                    <th >
                        
                    </th>
                    <th >
                        Copy
                    </th>
                    <th >
                        Rename
                    </th>
        
                    <th >
                        Delete
                    </th>
                </tr>
            </thead>
            <tbody>
                <tr
                    v-for="name in db_names"
                    :key="name"
                >
                    <td>
                        <v-btn class="text-none" @click="setDBName(name)" :color="name == db_name ? 'primary' : 'grey'">{{ name }}</v-btn>
                    </td>
                   
                    
                </tr>
            </tbody>
        </v-table> -->
    </v-sheet>
</template>

<style>
    th{
        text-align: left;
    }
</style>