<script setup lang="ts">
import {ref} from "vue";
    const props = defineProps(["date_str", "name"])
    const emit = defineEmits(["sendDate"])
    const string_date = ref(props.date_str != undefined ? props.date_str : "")
    const cal_date = ref(props.date_str != undefined ? new Date(props.date_str) : undefined)
    const dialog = ref(false)
        
    function emitDate(){
        string_date.value = cal_date.value == undefined ? "" : cal_date.value.toISOString().slice(0,10)
        emit("sendDate", string_date)
        dialog.value = false
    }
    function openDialog(){
        dialog.value = true
        cal_date.value = props.date_str != undefined ? new Date(props.date_str) : new Date()
        string_date.value = props.date_str != undefined ? props.date_str : ""
    }
    function onClear(){
        string_date.value = ""
        emit("sendDate", string_date)
    }
     
</script>
<template>
        <v-text-field
        :label="name"
        :model-value="string_date"
        prepend-inner-icon="mdi-calendar"
        @click:prepend-inner="openDialog"
        persistent-clear
        readonly
        clearable
        @click:clear="onClear"
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
                        class="mx-auto mb-5 bg-primary"
                        text="Ok"
                        @click="emitDate"
                        ></v-btn>
                        <v-btn
                        class="mx-auto mb-5 bg-grey"
                        text="Discard"
                        @click="dialog=false"
                        ></v-btn>
                </v-layout>
            </v-card>
             
        </v-dialog>
</template>
