<template>
    <v-data-table-virtual
        :headers="headers"
        :items="virtualItems"
        height="100dvh"
        item-value="name"
    >
        <template v-slot:item.edit="{item}">
            <v-icon @click="deleteItem(item)">mdi-pencil</v-icon>
        </template>
        <template v-slot:item.del="{item}">
            <v-icon @click="editItem(item)">mdi-delete</v-icon>
        </template>
    </v-data-table-virtual>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { ref, reactive, computed } from "vue";
import { VDataTableVirtual, VBtn } from "vuetify/components";

    const id = ref(1);

    function deleteItem(item){
        return 0
    }
    function editItem(item){
        return 0
    }
    async function get_list(){
            res.value = await invoke("find_items_by_parent_id", {id: id.value})
        }
    const virtualItems = computed(()=>{
            let l = []
            const item = {
                    name: "name",
                    priority: 0,
                    estTime: Date.now(),
                    link: "https://www.google.com/",
                    availability: Date(0),
                    completed: "No",
                    edit: "e but",
                    del: "del but",
                }
            for (let i =0; i < 20; i++){
                    let item_copy = {...item}
                    item_copy.priority = i
                    item_copy.name = `${item_copy.name}#${i}`
                    l.push(item_copy)
                }
            return l
        }
    )

    // headers cannot be reactive, but if I wanted a val that would update virtual items when changed I would use reactive() instead of ref().
    const headers = ref([
            { title: 'Name', align: 'start', key: 'name' },
            { title: 'Priority', align: 'end', key: 'priority' },
            { title: 'Est Time', align: 'end', key: 'estTime' },
            { title: 'Link', align: 'end', key: 'link' },
            { title: 'Available', align: 'end', key: 'availability' },
            { title: 'Completed', align: 'end', key:'completed'},
            { title: 'Edit', align: 'end', key: 'edit' },
            { title: "Del", align: "end", key: "del"},
    ])

// export default {
//     setup(){
//         const id = ref(1);
//         const res = ref("")
//         return {id, res}
//     },
//     data () {
//         return {
//         headers: [
//             { title: 'Name', align: 'start', key: 'name' },
//             { title: 'Priority', align: 'end', key: 'priority' },
//             { title: 'Est Time', align: 'end', key: 'estTime' },
//             { title: 'Link', align: 'end', key: 'link' },
//             { title: 'Edit', align: 'end', key: 'edit' },
//             { titel: "Del", align: "end", key: "del"},
//         ],
//         }
//     },

//     computed: {
//         virtualItems() {
//             let l = []
//             const item = {
//                     name: "name",
//                     priority: 0,
//                     estTime: Date(25),
//                     link: "https://www.google.com/",
//                     edit: "e but",
//                     del: "del but"
//                 }
//             for (let i =0; i< 20; i++){
                
//                     l.push({...item})
//                 }
//             return l
//         },
//     },
//     methods:{
//         get_list: async function(){
//             res.value = await invoke("find_items_by_parent_id", {id: id.value})
//         },
//         do_thing: function(){
//             res.value += "a"
//         } 
// }
// }
</script>

