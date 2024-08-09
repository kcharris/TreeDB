<script setup lang="ts">
import { ref, watch} from "vue";

    const emit = defineEmits(["copy"])
    const props = defineProps(['db_name'])
    const dialog = ref(false)
    const new_db_name = ref("")

    function emitCopy(){
        emit("copy", {dbName: props.db_name, cloneName: new_db_name.value})
        dialog.value = false
    }

    watch(dialog, ()=>{
        if (dialog){
            new_db_name.value = props.db_name + "Copy"
        }
    })
</script>

<template>
    <v-btn @click="dialog=true" color="info" icon="mdi-database-plus"></v-btn>

    <v-dialog
        v-model="dialog"
        max-width="500"
        >
        <v-card>
            <v-card-title>Database Cloning</v-card-title>
            <v-card-text>Enter a different name for the copy of the database. If a database shares the name of an existing DB, nothing will happen.</v-card-text>
            <v-card-item>
                <v-text-field v-model="new_db_name"/>
            </v-card-item>
            
            <v-card-actions>
                <v-btn
                class="bg-primary"
                text="Create"
                @click="emitCopy"
                ></v-btn>
                <v-btn
                    class="bg-grey"
                    text="Cancel"
                    @click="dialog=false"
                ></v-btn>
            </v-card-actions>
            
        </v-card>
            
    </v-dialog>
</template>
