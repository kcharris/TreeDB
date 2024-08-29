<script setup lang="ts">
import { ref, watch} from "vue";

    const emit = defineEmits(["delete"])
    const props = defineProps(['tag'])
    const dialog = ref(false)
    const delete_verification = ref("")

    function emitDelete(){
        if (delete_verification.value == props.tag.name){
            emit("delete", props.tag.id)
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
            <v-card-title>Tag Deletion</v-card-title>
            <v-card-subtitle>{{ props.tag.name }}</v-card-subtitle>
            <v-card-text>Are you sure you want to delete this Tag? Enter it's name to verify before deletion.</v-card-text>
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
