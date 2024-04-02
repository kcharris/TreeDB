<script setup>
import {ref, reactive} from "vue";
    defineProps({
        name: String
    })
    const dialog = ref(false)
    const date = ref(new Date());
    
    // defineEmits(["sendDate"])
    function emitDate(){
        dialog = false
        // $emit("sendDate", date)
    }
    
  
</script>
<template>
    <div>
        <v-text-field
        :label="name"
        v-model="date"
        hint="Use YYYY-MM-DD format"
        prepend-inner-icon="mdi-calendar"
        @click:prepend-inner="dialog = true"
        >
        </v-text-field>
        <v-dialog
            v-model="dialog"
            max-width="500"
            >
            <v-card>
                <v-date-picker v-model="date"></v-date-picker>
            </v-card>
            <template v-slot:actions>
                <v-btn
                class="ms-auto"
                text="Ok"
                @click="emitDate"
                ></v-btn>
            </template>   
        </v-dialog>
    </div>
</template>
