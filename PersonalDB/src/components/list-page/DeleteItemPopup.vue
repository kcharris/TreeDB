<script setup lang="ts">
import { ref, watch} from "vue";

    const emit = defineEmits(["delete"])
    const dialog = ref(false)
    const delete_verification = ref("")

    function emitDelete(){
        if (delete_verification.value == "y"){
            emit("delete")
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
    <v-icon
    icon="mdi-delete"
    @click ="dialog = true"
    >
    </v-icon>
    <v-dialog
        v-model="dialog"
        max-width="500"
        >
        <v-card >
            <v-card-title>Item Deletion</v-card-title>
            <v-card-text>Are you sure you want to delete this item? All sub-items will also be deleted. Type 'y' into the field below to confirm deletion.</v-card-text>
            <v-card-item>
                <v-text-field v-model="delete_verification" placeholder="Type y here to confirm"></v-text-field>
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
