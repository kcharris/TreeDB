<script setup lang="ts">
import {ref, watch} from "vue";
import { useDate } from 'vuetify'
    const props = defineProps(["date_str", "name"])
    const emit = defineEmits(["sendDate"])
    const u_date = useDate()
    const string_date = ref(props.date_str != undefined ? props.date_str : "")
    const cal_date = ref(new Date())
    const dialog = ref(false)
    
    watch(cal_date, () => {string_date.value = u_date.format(cal_date.value, "keyboardDate")})
    
    function emitDate(){
        emit("sendDate", string_date)
        dialog.value = false
    }
    function openDialog(){
        dialog.value = true
        cal_date.value = new Date()
        string_date.value = ""
    }
    
  
</script>
<template>
    <div>
        <v-text-field
        :label="name"
        :model-value="string_date"
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
                <v-date-picker v-model="cal_date"></v-date-picker>
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
