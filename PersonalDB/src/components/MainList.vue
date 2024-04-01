<template>
    <!-- <v-data-table-virtual
        :headers="headers"
        :items="virtualItems"
        height="60dvh"
        item-value="name"
    ></v-data-table-virtual> -->
    <v_btn @click="get_list">Press me</v_btn>
    <v_btn @click="do_thing">Press me</v_btn>
    <p>{{ res }} and more</p>
</template>

<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

    const id = ref(1);
    const res = ref("")

    async function get_list(){
        res.value = await invoke("find_items_by_parent_id", {id: id.value})
    }
    function do_thing(){
        res.value += "a"
    }

        // export default {
        //     data () {
        //         return {
        //         headers: [
        //             { title: 'Name', align: 'start', key: 'name' },
        //             { title: 'Priority', align: 'end', key: 'priority' },
        //             { title: 'Est Time', align: 'end', key: 'estTime' },
        //             { title: 'Link', align: 'end', key: 'link' },
        //             { title: 'Edit', align: 'end', key: 'edit' },
        //             { titel: "Del", align: "end", key: "del"},
        //         ]
        //         }
        // },

        // // computed: {
        // //     virtualItems () {
        // //         let json_list = get_list()
                
        // //     return json_list // a json list containing the information above, revieced from a call to the database using rust
        // //     // and stored and sent through a struct.
        // //     },
        // // },
        // // methods:{
        // //     get_list
        // // }
        // }
</script>
