<script setup lang="ts">
    import { VTable, VSheet} from "vuetify/components";
    import { ref, onMounted} from "vue";
    import { invoke } from "@tauri-apps/api/tauri";
    import CreatePopup from "./CreatePopup.vue"
    import RenamePopup from "./RenamePopup.vue"
    import DeletePopup from "./DeletePopup.vue"
    import {Tag} from "../../item-types"

    onMounted(()=>{
        refresh_tags();
    })

    const db_name = defineModel("dbName")
    const tags = defineModel<Tag[]>("tags")
    const tag_names = ref<Set<string>>()

    async function refresh_tags(){
        let tags_str:string = await invoke("get_tags", {dbName: db_name.value})
        tags.value = tags_str == "" ? [] : JSON.parse(tags_str)
        tag_names.value = new Set(tags.value?.map((t)=> {return t.name.toLocaleLowerCase()}))
    }

    async function createTag(name: string){
        if (!tag_names.value?.has(name.toLocaleLowerCase())){
            await invoke("add_tag", {dbName: db_name.value, name: name});
            refresh_tags();
        }
    }

    async function renameTag(tag: Tag){
        if (!tag_names.value?.has(tag.name.toLocaleLowerCase())){
            let payload = JSON.stringify(tag)
            await invoke("update_tag", {dbName: db_name.value, payload: payload})
            refresh_tags();
        } 
    }

    async function deleteTag(id: Number){
        await invoke("delete_tag", {dbName: db_name.value, id: id })
        refresh_tags();
    }

</script>


<template>    
    <v-sheet color="teal-lighten-4" class="fill-height mx-auto w-100">
        <v-container>
            <v-toolbar color="blue-grey-lighten-5 w-50 mx-auto" density="compact">
                <p class="ml-5">Tag Manager</p>
                <v-spacer/>
                <CreatePopup @create="createTag"></CreatePopup>
            </v-toolbar> 
            <v-card v-if="tags?.length == 0" class="w-50 h-50 mx-auto mt-10">
                <v-card-title>No tags found</v-card-title>
            </v-card>
            <v-table height="85dvh" v-else class="w-50 overflow-x-auto mx-auto">
                <thead>
                    <tr>
                        <th >
                            Tag Name
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
                        v-for="tag in tags"
                        :key="tag.name"
                    >
                        <td>
                            <p>{{ tag.name }}</p>
                        </td>
                        <td>
                            <RenamePopup :tag="tag" @rename="renameTag"></RenamePopup>
                        </td>
                        
                        <td>
                            <DeletePopup :tag="tag" @delete="deleteTag"></DeletePopup>
                        </td>
                    </tr>
                </tbody>
            </v-table>
        </v-container>
    </v-sheet>
</template>

<style>
    th{
        text-align: left;
    }
</style>