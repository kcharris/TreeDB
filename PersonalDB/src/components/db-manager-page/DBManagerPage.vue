<script setup lang="ts">
    import { VTable, VBtn, VSheet} from "vuetify/components";
    import { ref, onMounted} from "vue";
    import { invoke } from "@tauri-apps/api/tauri";
    import BackupResorePopup from "./BackupRestorePopup.vue"
    import CreatePopup from "./CreatePopup.vue"
    import CopyPopup from "./CopyPopup.vue"
    import RenamePopup from "./RenamePopup.vue"
    import DeletePopup from "./DeletePopup.vue"

    onMounted(()=>{
        refresh_db_filenames();
    })

    const db_name = defineModel()
    const db_names = ref<string[]>([])

    async function refresh_db_filenames(){
        let filenames: string[] = await invoke("get_db_filenames")
        db_names.value = filenames
    }

    function setDBName(name: string){
        db_name.value = name;
    }
    
    async function createDB(name: string){
        await invoke("create_db_file", {dbName: name});
        refresh_db_filenames();
    }

    function renameDB(payload: any){
        invoke("rename_db", payload)
        refresh_db_filenames();
    }

    async function deleteDB(name: string){
        await invoke("delete_db_file", {dbName: name})
        refresh_db_filenames();
    }

    async function copyDB(payload: any){
        await invoke("clone_db_file", payload);
        refresh_db_filenames();
    }

</script>


<template>    
    <p>{{ db_name }}</p>
    <v-toolbar color="blue-grey-lighten-5" density="compact">
    <v-spacer/>
    <CreatePopup @create="createDB"></CreatePopup>
    </v-toolbar>

    <v-sheet color="teal-lighten-2" class="fill-height mx-auto w-100">
        <v-table class="w-75 fill-height overflow-x-auto mx-auto">
            <thead>
                <tr>
                    <th >
                        DB Select
                    </th>
                    <th >
                        Backup/Restore
                    </th>
                    <th >
                        Copy
                    </th>
                    <th >
                        Rename
                    </th>
                    <!-- <th >
                        CSV Export
                    </th> -->
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
                    <td>
                        <BackupResorePopup :db_name="name"></BackupResorePopup>
                    </td>
                    <td>
                        <CopyPopup :db_name="name" @copy="copyDB"></CopyPopup>
                    </td>
                    <td>
                        <RenamePopup :db_name="name" @rename="renameDB"></RenamePopup>
                    </td>
                    <!-- <td>
                        <v-btn color="lime-darken-3" icon="mdi-database-export"></v-btn>
                    </td> -->
                    <td>
                        <DeletePopup :db_name="name" @delete="deleteDB"></DeletePopup>
                    </td>
                </tr>
            </tbody>
        </v-table>
    </v-sheet>
</template>

<style>
    th{
        text-align: left;
    }
</style>