<script setup>
import {ref, reactive, computed, defineModel} from "vue";
import { useDate } from 'vuetify'

    const uDate = useDate()
    const calDate = ref(new Date())
    
    defineProps({
        name: String
    })
    const dialog = ref(false)
    
    // defineEmits(["sendDate"])
    function emitDate(){
        dialog = false
        // $emit("sendDate", date)
    }
    const getTextDate = computed(() =>{
        return uDate.format(calDate.value, "keyboardDate")
    })
    
  
</script>
<template>
    <div>
        <v-text-field
        :label="name"
        :model-value="getTextDate"
        hint="Use MM/DD/YYYY format"
        prepend-inner-icon="mdi-calendar"
        @click:prepend-inner="dialog = true"
        >
        </v-text-field>
        <v-dialog
            v-model="dialog"
            max-width="500"
            >
            <v-card>
                <v-date-picker v-model="calDate"></v-date-picker>
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
