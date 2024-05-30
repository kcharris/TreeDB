<script setup lang="ts">
import {ref} from "vue";
import { useDate } from 'vuetify'
    const props = defineProps(["date_str", "name"])
    const emit = defineEmits(["sendDate"])
    const u_date = useDate()
    const string_date = ref(props.date_str != undefined ? props.date_str : "")
    const cal_date = ref(props.date_str != undefined ? new Date(props.date_str) : "")
    const dialog = ref(false)
        
    function emitDate(){
        string_date.value = u_date.format(cal_date.value, "keyboardDate")
        emit("sendDate", string_date)
        dialog.value = false
    }
    function openDialog(){
        dialog.value = true
        cal_date.value = props.date_str != undefined ? new Date(props.date_str) : new Date()
        string_date.value = props.date_str != undefined ? props.date_str : ""
    }
     
</script>
<template>
        <v-text-field
        :label="name"
        :model-value="string_date"
        prepend-inner-icon="mdi-calendar"
        @click:prepend-inner="openDialog"
        readonly
        clearable
        >
        </v-text-field>
        <v-dialog
            v-model="dialog"
            max-width="500"
            >
            <v-card>
                <v-layout class="d-flex flex-column">
                    <v-date-picker class="mx-auto" v-model="cal_date"></v-date-picker>
                        <v-btn
                        class="mx-auto mb-10 bg-primary"
                        text="Ok"
                        @click="emitDate"
                        ></v-btn>
                </v-layout>
            </v-card>
             
        </v-dialog>
</template>
