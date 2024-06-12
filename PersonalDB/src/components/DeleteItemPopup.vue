<script setup lang="ts">
import { ref} from "vue";

    const emit = defineEmits(["delete"])
    const dialog = ref(false)

    function emitDelete(){
        emit("delete")
        dialog.value = false
    }
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
        <v-card>
            <template v-slot:actions>
            <p>Are you sure you want to delete?</p>
            <v-btn
            class="ms-auto"
            text="Yes"
            @click="emitDelete"
            ></v-btn>
            <v-btn
                class="ms-auto"
                text="Back"
                @click="dialog=false"
            ></v-btn>
        </template>  
        </v-card>
            
    </v-dialog>
</template>
