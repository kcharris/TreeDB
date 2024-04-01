<template>
    <v-data-table-virtual
        :headers="headers"
        :items="virtualItems"
        height="60dvh"
        item-value="name"
    ></v-data-table-virtual>
</template>

<script>
    export default {
    data () {
        return {
        headers: [
            { title: 'Name', align: 'start', key: 'name' },
            { title: 'Priority', align: 'end', key: 'priority' },
            { title: 'Est Time', align: 'end', key: 'estTime' },
            { title: 'Link', align: 'end', key: 'link' },
            { title: 'Edit', align: 'end', key: 'edit' },
            { titel: "Del", align: "end", key: "del"},
        ]
        }
    },

    computed: {
        virtualItems () {
            // here use rust to get a vec of items
            
        return [0] // a json list containing the information above, revieced from a call to the database using rust
        // and stored and sent through a struct.
        },
    },
    }
</script>