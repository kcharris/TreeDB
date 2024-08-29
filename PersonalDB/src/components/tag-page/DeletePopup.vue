<script setup lang="ts">
import { ref, watch} from "vue";

    const emit = defineEmits(["delete"])
    const props = defineProps(['db_name'])
    const dialog = ref(false)
    const delete_verification = ref("")

    function emitDelete(){
        if (delete_verification.value == props.db_name){
            emit("delete", props.db_name)
            dialog.value = false
        }
    }

    watch(dialog, ()=>{
        if (dialog.value == true){
            delete_verification.value = ""
        }
    })
</script>

<template>
    <v-btn @click="dialog=true" color="red-darken-4" icon="mdi-delete"></v-btn>

    <v-dialog
        v-model="dialog"
        max-width="500"
        >
        <v-card >
            <v-card-title>Database Deletion</v-card-title>
            <v-card-subtitle>{{ props.db_name }}</v-card-subtitle>
            <v-card-text>Are you sure you want to delete this database? Enter it's name to verify before deletion.</v-card-text>
            <v-card-item>
                <v-text-field v-model="delete_verification"></v-text-field>
            </v-card-item>
            <v-card-actions>
                <v-btn
                class="bg-red-darken-4"
                text="Delete"
                @click="emitDelete"
                ></v-btn>
                <v-btn
                    class="justify-end bg-primary"
                    text="Back"
                    @click="dialog=false"
                ></v-btn>
            </v-card-actions>
        </v-card>
            
    </v-dialog>
</template>
