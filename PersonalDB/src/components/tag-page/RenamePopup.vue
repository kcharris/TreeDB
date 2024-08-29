<script setup lang="ts">
import { ref, watch} from "vue";
import {Tag} from "../../item-types"

    const emit = defineEmits(["rename"])
    const props = defineProps(['tag'])
    const dialog = ref(false)
    const updated_tag = ref<Tag>({name:"", id: -1})

    function emitRename(){
        if (updated_tag.value.name != ""){
            emit("rename", updated_tag.value)
            dialog.value = false
        }
    }

    watch(dialog, ()=>{
        if (dialog){
            updated_tag.value.id = props.tag.id
            updated_tag.value.name = ""
        }
    })

</script>

<template>
    <v-btn @click="dialog=true" color="indigo-lighten-1" icon="mdi-square-edit-outline"></v-btn>

    <v-dialog
        v-model="dialog"
        max-width="500"
        >
        <v-card>
            <v-card-title>Rename Tag</v-card-title>
            <v-card-text>Enter a different name for the Tag. If the new name shares the name of an existing Tag, nothing will happen.</v-card-text>
            <v-card-item>
                <v-text-field v-model="updated_tag.name"/>
            </v-card-item>
            
            <v-card-actions>
                <v-btn
                class="bg-primary"
                text="Rename"
                @click="emitRename"
                ></v-btn>
                <v-btn
                    class="bg-grey"
                    text="Cancel"
                    @click="dialog=false"
                ></v-btn>
            </v-card-actions>
            
        </v-card>
            
    </v-dialog>
</template>