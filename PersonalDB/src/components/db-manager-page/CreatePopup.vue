<script setup lang="ts">
import { ref, watch} from "vue";

    const emit = defineEmits(["create"])
    const dialog = ref(false)
    const new_db_name = ref("")

    function emitCreate(){
        emit("create", new_db_name.value)
        dialog.value = false
    }

    watch(dialog, ()=>{
        if (dialog){
            new_db_name.value = ""
        }
    })
</script>

<template>
    <v-btn @click="dialog=true" class="bg-primary mr-5">Create New DB</v-btn>
    
    <v-dialog
        v-model="dialog"
        max-width="500"
        >
        <v-card>
            <v-card-title>Database Creation</v-card-title>
            <v-card-text>Enter a name for the database to create. If a database shares the name of an existing DB, nothing will happen.</v-card-text>
            <v-card-item>
                <v-text-field v-model="new_db_name"/>
            </v-card-item>
            
            <v-card-actions>
                <v-btn
                class="bg-primary"
                text="Create"
                @click="emitCreate"
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