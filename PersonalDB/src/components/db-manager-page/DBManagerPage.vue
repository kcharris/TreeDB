<script setup lang="ts">
    import { VTable, VBtn, VSheet} from "vuetify/components";
    import { ref} from "vue";

    const db_name = defineModel()
    const db_names = ref(['default', 'placeholder'])

    function setDBName(name: string){
        db_name.value = name;
    }

</script>


<template>
    <p>{{ db_name }}</p>
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
                        <v-btn @click="setDBName(name)" :color="name == db_name ? 'primary' : 'grey'">{{ name }}</v-btn>
                    </td>
                    <td>
                        <v-btn color="warning" icon="mdi-database-cog"></v-btn>
                    </td>
                    <td>
                        <v-btn color="info" icon="mdi-database-plus"></v-btn>
                    </td>
                    <td>
                        <v-btn color="indigo-lighten-1" icon="mdi-square-edit-outline"></v-btn>
                    </td>
                    <!-- <td>
                        <v-btn color="lime-darken-3" icon="mdi-database-export"></v-btn>
                    </td> -->
                    <td>
                        <v-btn color="red-darken-4" icon="mdi-delete"></v-btn>
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