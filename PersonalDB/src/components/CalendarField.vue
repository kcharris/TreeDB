<script setup lang="ts">
import {ref, watch} from "vue";
import { useDate } from 'vuetify'

    const emit = defineEmits(["sendDate"])
    const uDate = useDate()
    const stringDate = ref("")
    const calDate = ref(new Date())
    const dialog = ref(false)
    
    defineProps({
        name: String
    })
    
    watch(calDate, () => {stringDate.value = uDate.format(calDate.value, "keyboardDate")})
    
    function emitDate(){
        emit("sendDate", stringDate)
        dialog.value = false
    }
    function openDialog(){
        dialog.value = true
        calDate.value = new Date()
        stringDate.value = ""
    }
    
  
</script>
<template>
    <div>
        <v-text-field
        :label="name"
        :model-value="stringDate"
        prepend-inner-icon="mdi-calendar"
        @click:prepend-inner="openDialog"
        readonly
        >
        </v-text-field>
        <v-dialog
            v-model="dialog"
            max-width="500"
            >
            <v-card>
                <v-date-picker v-model="calDate"></v-date-picker>
                <template v-slot:actions>
                <v-btn
                class="ms-auto"
                text="Ok"
                @click="emitDate"
                ></v-btn>
            </template>  
            </v-card>
             
        </v-dialog>
    </div>
</template>
