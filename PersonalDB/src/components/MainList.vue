<template>
    <v-data-table-virtual
        :headers="headers"
        :items="virtualItems"
        height="60dvh"
        item-value="name"
    ></v-data-table-virtual>
    <v-btn @click="get_list">Press me</v-btn>
    <v-btn @click="do_thing">Press me</v-btn>
    <p>{{ res }} and more</p>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { ref, reactive, computed } from "vue";
import { VDataTableVirtual, VBtn } from "vuetify/components";

    const id = ref(1);
    const res = ref("")

    async function get_list(){
            res.value = await invoke("find_items_by_parent_id", {id: id.value})
        }
    function do_thing(){
        res.value += "a"
    } 
    const virtualItems = computed(()=>{
            let l = []
            const item = {
                    name: "name",
                    priority: 0,
                    estTime: Date(25),
                    link: "https://www.google.com/",
                    edit: "e but",
                    del: "del but"
                }
            for (let i =0; i< 20; i++){
                
                    l.push({...item})
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
            { title: 'Edit', align: 'end', key: 'edit' },
            { titel: "Del", align: "end", key: "del"},
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

