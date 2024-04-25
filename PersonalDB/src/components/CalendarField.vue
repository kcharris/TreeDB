<script setup lang="ts">
import {ref, reactive, computed, defineModel} from "vue";
import { useDate } from 'vuetify'

    const emit = defineEmits(["sendDate"])
    const uDate = useDate()
    const calDate = ref(new Date())
    
    defineProps({
        name: String
    })
    const dialog = ref(false)
    
    function emitDate(){
        emit("sendDate", getTextDate)
        dialog.value = false
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
